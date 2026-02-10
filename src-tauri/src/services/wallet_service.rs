use crate::error::AppError;
use crate::models::params::CreateWalletParams;
use crate::models::wallet::Wallet;
use sqlx::SqlitePool;

pub async fn create_wallet(
    pool: &SqlitePool,
    params: &CreateWalletParams,
) -> Result<Wallet, AppError> {
    let name = params.name.trim();
    if name.is_empty() || name.len() > 50 {
        return Err(AppError::validation("Wallet name must be 1-50 characters"));
    }

    // Use a SQLite transaction
    let mut tx = pool.begin().await?;

    let wallet = sqlx::query_as::<_, Wallet>(
        "INSERT INTO wallets (family_id, name, type, balance_cents, currency, warning_threshold_cents) \
         VALUES (?, ?, ?, 0, 'TWD', 0) RETURNING *"
    )
    .bind(params.family_id)
    .bind(name)
    .bind(&params.wallet_type)
    .fetch_one(&mut *tx).await
    .map_err(|e| {
        if e.to_string().contains("UNIQUE constraint failed") {
            AppError::new("WALLET_NAME_EXISTS", "A wallet with this name already exists")
        } else {
            AppError::from(e)
        }
    })?;

    // If initial balance is provided, create a manual credit transaction
    if let Some(initial) = params.initial_balance_cents
        && initial > 0
    {
        sqlx::query(
            "INSERT INTO transactions (family_id, wallet_id, type, amount_cents, source_type, occurred_at) \
             VALUES (?, ?, 'credit', ?, 'manual', datetime('now'))"
        )
        .bind(params.family_id)
        .bind(wallet.id)
        .bind(initial)
        .execute(&mut *tx).await?;

        sqlx::query("UPDATE wallets SET balance_cents = ? WHERE id = ?")
            .bind(initial)
            .bind(wallet.id)
            .execute(&mut *tx)
            .await?;
    }

    tx.commit().await?;

    // Re-fetch to get updated balance
    let wallet = sqlx::query_as::<_, Wallet>("SELECT * FROM wallets WHERE id = ?")
        .bind(wallet.id)
        .fetch_one(pool)
        .await?;

    Ok(wallet)
}

pub async fn list_wallets(pool: &SqlitePool, family_id: i64) -> Result<Vec<Wallet>, AppError> {
    let wallets = sqlx::query_as::<_, Wallet>(
        "SELECT * FROM wallets WHERE family_id = ? AND is_deleted = 0 ORDER BY created_at ASC",
    )
    .bind(family_id)
    .fetch_all(pool)
    .await?;
    Ok(wallets)
}

pub async fn get_wallet(pool: &SqlitePool, id: i64) -> Result<Wallet, AppError> {
    sqlx::query_as::<_, Wallet>("SELECT * FROM wallets WHERE id = ? AND is_deleted = 0")
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::not_found("wallet"))
}

pub async fn update_wallet(
    pool: &SqlitePool,
    id: i64,
    name: &str,
    warning_threshold_cents: i64,
) -> Result<Wallet, AppError> {
    let name = name.trim();
    if name.is_empty() || name.len() > 50 {
        return Err(AppError::validation("Wallet name must be 1-50 characters"));
    }
    sqlx::query(
        "UPDATE wallets SET name = ?, warning_threshold_cents = ?, sync_version = sync_version + 1, updated_at = datetime('now') WHERE id = ? AND is_deleted = 0"
    )
    .bind(name)
    .bind(warning_threshold_cents)
    .bind(id)
    .execute(pool).await?;

    get_wallet(pool, id).await
}

pub async fn archive_wallet(pool: &SqlitePool, id: i64) -> Result<(), AppError> {
    let wallet = get_wallet(pool, id).await?;
    if wallet.status == "archived" {
        return Err(AppError::new(
            "WALLET_ARCHIVED",
            "Wallet is already archived",
        ));
    }
    sqlx::query(
        "UPDATE wallets SET status = 'archived', sync_version = sync_version + 1, updated_at = datetime('now') WHERE id = ?"
    )
    .bind(id)
    .execute(pool).await?;
    Ok(())
}
