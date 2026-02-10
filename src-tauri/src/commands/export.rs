use crate::error::AppError;
use crate::services::export_service;
use sqlx::SqlitePool;
use tauri::State;

#[tauri::command]
pub async fn export_csv(
    family_id: i64,
    date_from: Option<String>,
    date_to: Option<String>,
    wallet_id: Option<i64>,
    pool: State<'_, SqlitePool>,
) -> Result<String, AppError> {
    export_service::export_transactions_csv(&pool, family_id, date_from, date_to, wallet_id).await
}
