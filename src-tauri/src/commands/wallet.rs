use crate::error::AppError;
use crate::models::params::CreateWalletParams;
use crate::models::wallet::Wallet;
use crate::services::wallet_service;
use sqlx::SqlitePool;
use tauri::State;

#[tauri::command]
pub async fn create_wallet(
    params: CreateWalletParams,
    pool: State<'_, SqlitePool>,
) -> Result<Wallet, AppError> {
    wallet_service::create_wallet(&pool, &params).await
}

#[tauri::command]
pub async fn list_wallets(
    family_id: i64,
    pool: State<'_, SqlitePool>,
) -> Result<Vec<Wallet>, AppError> {
    wallet_service::list_wallets(&pool, family_id).await
}

#[tauri::command]
pub async fn get_wallet(id: i64, pool: State<'_, SqlitePool>) -> Result<Wallet, AppError> {
    wallet_service::get_wallet(&pool, id).await
}

#[tauri::command]
pub async fn update_wallet(
    id: i64,
    name: String,
    warning_threshold_cents: i64,
    pool: State<'_, SqlitePool>,
) -> Result<Wallet, AppError> {
    wallet_service::update_wallet(&pool, id, &name, warning_threshold_cents).await
}

#[tauri::command]
pub async fn archive_wallet(id: i64, pool: State<'_, SqlitePool>) -> Result<(), AppError> {
    wallet_service::archive_wallet(&pool, id).await
}
