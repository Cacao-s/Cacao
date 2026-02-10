use crate::error::AppError;
use crate::services::auth_service;
use sqlx::SqlitePool;
use tauri::State;

#[tauri::command]
pub async fn set_pin(pin: String, pool: State<'_, SqlitePool>) -> Result<(), AppError> {
    auth_service::set_pin(&pool, &pin).await
}

#[tauri::command]
pub async fn verify_pin(pin: String, pool: State<'_, SqlitePool>) -> Result<bool, AppError> {
    auth_service::verify_pin(&pool, &pin).await
}

#[tauri::command]
pub async fn remove_pin(pool: State<'_, SqlitePool>) -> Result<(), AppError> {
    auth_service::remove_pin(&pool).await
}

#[tauri::command]
pub async fn has_pin(pool: State<'_, SqlitePool>) -> Result<bool, AppError> {
    auth_service::has_pin(&pool).await
}
