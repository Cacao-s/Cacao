use crate::error::AppError;
use rand::Rng;
use sqlx::SqlitePool;

pub async fn generate_pairing_code(pool: &SqlitePool, family_id: i64) -> Result<String, AppError> {
    let code: u32 = rand::thread_rng().gen_range(100000..999999);
    let code_str = code.to_string();

    // Store code and expiry in sync_state (5 min expiry)
    sqlx::query("INSERT OR REPLACE INTO sync_state (key, value) VALUES ('pairing_code', ?)")
        .bind(&code_str)
        .execute(pool)
        .await?;

    let expires = chrono::Utc::now() + chrono::Duration::minutes(5);
    sqlx::query("INSERT OR REPLACE INTO sync_state (key, value) VALUES ('pairing_expires_at', ?)")
        .bind(expires.to_rfc3339())
        .execute(pool)
        .await?;

    sqlx::query("INSERT OR REPLACE INTO sync_state (key, value) VALUES ('pairing_family_id', ?)")
        .bind(family_id.to_string())
        .execute(pool)
        .await?;

    Ok(code_str)
}

pub async fn validate_pairing_code(pool: &SqlitePool, code: &str) -> Result<i64, AppError> {
    // Get stored code
    let stored =
        sqlx::query_scalar::<_, String>("SELECT value FROM sync_state WHERE key = 'pairing_code'")
            .fetch_optional(pool)
            .await?
            .ok_or_else(|| AppError::new("PAIRING_CODE_INVALID", "Invalid pairing code"))?;

    if stored != code {
        return Err(AppError::new(
            "PAIRING_CODE_INVALID",
            "Invalid pairing code",
        ));
    }

    // Check expiry
    let expires_str = sqlx::query_scalar::<_, String>(
        "SELECT value FROM sync_state WHERE key = 'pairing_expires_at'",
    )
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::new("PAIRING_CODE_EXPIRED", "Pairing code has expired"))?;

    let expires = chrono::DateTime::parse_from_rfc3339(&expires_str)
        .map_err(|_| AppError::internal("Invalid expiry format"))?;

    if chrono::Utc::now() > expires {
        // Clean up expired code
        sqlx::query("DELETE FROM sync_state WHERE key IN ('pairing_code', 'pairing_expires_at', 'pairing_family_id')")
            .execute(pool).await?;
        return Err(AppError::new(
            "PAIRING_CODE_EXPIRED",
            "Pairing code has expired",
        ));
    }

    // Get family_id
    let family_id_str = sqlx::query_scalar::<_, String>(
        "SELECT value FROM sync_state WHERE key = 'pairing_family_id'",
    )
    .fetch_one(pool)
    .await?;
    let family_id: i64 = family_id_str
        .parse()
        .map_err(|_| AppError::internal("Invalid family_id"))?;

    // Clean up used code
    sqlx::query("DELETE FROM sync_state WHERE key IN ('pairing_code', 'pairing_expires_at', 'pairing_family_id')")
        .execute(pool).await?;

    Ok(family_id)
}
