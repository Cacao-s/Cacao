use crate::error::AppError;
use crate::models::audit_log::AuditLog;
use sqlx::SqlitePool;

pub async fn list_audit_logs(
    pool: &SqlitePool,
    family_id: i64,
    limit: Option<i64>,
    offset: Option<i64>,
) -> Result<Vec<AuditLog>, AppError> {
    let limit = limit.unwrap_or(50);
    let offset = offset.unwrap_or(0);

    let audit_logs = sqlx::query_as::<_, AuditLog>(
        "SELECT * FROM audit_logs WHERE family_id = ? ORDER BY created_at DESC LIMIT ? OFFSET ?",
    )
    .bind(family_id)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await?;

    Ok(audit_logs)
}
