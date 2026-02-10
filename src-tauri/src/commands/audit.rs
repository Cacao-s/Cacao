use crate::error::AppError;
use crate::models::audit_log::AuditLog;
use crate::services::audit_service;
use sqlx::SqlitePool;
use tauri::State;

#[tauri::command]
pub async fn list_audit_logs(
    family_id: i64,
    limit: i64,
    offset: i64,
    pool: State<'_, SqlitePool>,
) -> Result<Vec<AuditLog>, AppError> {
    audit_service::list_audit_logs(&pool, family_id, Some(limit), Some(offset)).await
}
