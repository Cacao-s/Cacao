use crate::error::AppError;
use crate::models::notification::Notification;
use crate::services::notification_service;
use sqlx::SqlitePool;
use tauri::State;

#[tauri::command]
pub async fn list_notifications(
    limit: i64,
    offset: i64,
    pool: State<'_, SqlitePool>,
) -> Result<Vec<Notification>, AppError> {
    notification_service::list_notifications(&pool, Some(limit), Some(offset)).await
}

#[tauri::command]
pub async fn mark_notification_read(id: i64, pool: State<'_, SqlitePool>) -> Result<(), AppError> {
    notification_service::mark_read(&pool, id).await
}

#[tauri::command]
pub async fn mark_all_notifications_read(pool: State<'_, SqlitePool>) -> Result<(), AppError> {
    notification_service::mark_all_read(&pool).await
}

#[tauri::command]
pub async fn unread_notification_count(pool: State<'_, SqlitePool>) -> Result<i64, AppError> {
    notification_service::unread_count(&pool).await
}
