use crate::error::AppError;
use crate::models::params::{ManualTransactionParams, TransactionFilters};
use crate::models::transaction::Transaction;
use crate::services::transaction_service::{self, MonthlySummary};
use sqlx::SqlitePool;
use tauri::State;

#[tauri::command]
pub async fn list_transactions(
    family_id: i64,
    filters: TransactionFilters,
    pool: State<'_, SqlitePool>,
) -> Result<Vec<Transaction>, AppError> {
    transaction_service::list_transactions(&pool, family_id, &filters).await
}

#[tauri::command]
pub async fn create_manual_transaction(
    params: ManualTransactionParams,
    pool: State<'_, SqlitePool>,
) -> Result<Transaction, AppError> {
    transaction_service::create_manual_transaction(&pool, &params).await
}

#[tauri::command]
pub async fn get_monthly_summary(
    family_id: i64,
    year: i32,
    month: u32,
    pool: State<'_, SqlitePool>,
) -> Result<MonthlySummary, AppError> {
    transaction_service::get_monthly_summary(&pool, family_id, year, month).await
}
