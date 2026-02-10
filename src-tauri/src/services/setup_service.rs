use crate::error::AppError;
use crate::models::profile::Profile;
use sqlx::SqlitePool;

pub async fn setup_device(
    pool: &SqlitePool,
    display_name: &str,
    role: &str,
) -> Result<Profile, AppError> {
    // Check if already setup
    let existing = sqlx::query_as::<_, Profile>("SELECT * FROM profiles LIMIT 1")
        .fetch_optional(pool)
        .await?;
    if existing.is_some() {
        return Err(AppError::new(
            "DEVICE_ALREADY_SETUP",
            "Device is already set up",
        ));
    }
    // Validate role
    if role != "giver" && role != "baby" {
        return Err(AppError::validation("Role must be 'giver' or 'baby'"));
    }
    // Validate display_name length 1-50
    let name = display_name.trim();
    if name.is_empty() || name.len() > 50 {
        return Err(AppError::validation("Display name must be 1-50 characters"));
    }
    // Insert profile
    let profile = sqlx::query_as::<_, Profile>(
        "INSERT INTO profiles (display_name, role) VALUES (?, ?) RETURNING *",
    )
    .bind(name)
    .bind(role)
    .fetch_one(pool)
    .await?;
    Ok(profile)
}

pub async fn get_profile(pool: &SqlitePool) -> Result<Option<Profile>, AppError> {
    let profile =
        sqlx::query_as::<_, Profile>("SELECT * FROM profiles WHERE is_deleted = 0 LIMIT 1")
            .fetch_optional(pool)
            .await?;
    Ok(profile)
}

pub async fn is_setup(pool: &SqlitePool) -> Result<bool, AppError> {
    let profile = get_profile(pool).await?;
    Ok(profile.is_some())
}

pub async fn reset_device(pool: &SqlitePool) -> Result<(), AppError> {
    // Delete all data - order matters for foreign keys
    sqlx::query("DELETE FROM audit_logs").execute(pool).await?;
    sqlx::query("DELETE FROM notifications")
        .execute(pool)
        .await?;
    sqlx::query("DELETE FROM transactions")
        .execute(pool)
        .await?;
    sqlx::query("DELETE FROM requests").execute(pool).await?;
    sqlx::query("DELETE FROM allowances").execute(pool).await?;
    sqlx::query("DELETE FROM wallets").execute(pool).await?;
    sqlx::query("DELETE FROM paired_devices")
        .execute(pool)
        .await?;
    sqlx::query("DELETE FROM family_members")
        .execute(pool)
        .await?;
    sqlx::query("DELETE FROM families").execute(pool).await?;
    sqlx::query("DELETE FROM profiles").execute(pool).await?;
    sqlx::query("DELETE FROM sync_state").execute(pool).await?;
    Ok(())
}
