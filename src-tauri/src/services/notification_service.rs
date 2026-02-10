use crate::error::AppError;
use crate::models::notification::Notification;
use sqlx::SqlitePool;

pub async fn list_notifications(
    pool: &SqlitePool,
    limit: Option<i64>,
    offset: Option<i64>,
) -> Result<Vec<Notification>, AppError> {
    let limit = limit.unwrap_or(50);
    let offset = offset.unwrap_or(0);

    let notifications = sqlx::query_as::<_, Notification>(
        "SELECT * FROM notifications ORDER BY created_at DESC LIMIT ? OFFSET ?",
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await?;

    Ok(notifications)
}

pub async fn mark_read(pool: &SqlitePool, id: i64) -> Result<(), AppError> {
    let result =
        sqlx::query("UPDATE notifications SET is_read = 1, read_at = datetime('now') WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::not_found("notification"));
    }

    Ok(())
}

pub async fn mark_all_read(pool: &SqlitePool) -> Result<(), AppError> {
    sqlx::query(
        "UPDATE notifications SET is_read = 1, read_at = datetime('now') WHERE is_read = 0",
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn unread_count(pool: &SqlitePool) -> Result<i64, AppError> {
    let (count,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM notifications WHERE is_read = 0")
        .fetch_one(pool)
        .await?;

    Ok(count)
}
