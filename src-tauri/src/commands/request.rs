use crate::error::AppError;
use crate::models::params::{CreateRequestParams, UpdateRequestParams};
use crate::models::request::Request;
use crate::models::transaction::Transaction;
use crate::services::{request_service, transaction_service};
use sqlx::SqlitePool;
use tauri::State;

#[tauri::command]
pub async fn create_request(
    params: CreateRequestParams,
    pool: State<'_, SqlitePool>,
) -> Result<Request, AppError> {
    request_service::create_request(&pool, &params).await
}

#[tauri::command]
pub async fn update_request(
    id: i64,
    params: UpdateRequestParams,
    pool: State<'_, SqlitePool>,
) -> Result<Request, AppError> {
    request_service::update_request(&pool, id, &params).await
}

#[tauri::command]
pub async fn submit_request(id: i64, pool: State<'_, SqlitePool>) -> Result<Request, AppError> {
    request_service::submit_request(&pool, id).await
}

#[tauri::command]
pub async fn approve_request(
    id: i64,
    approver_member_id: i64,
    pool: State<'_, SqlitePool>,
) -> Result<Request, AppError> {
    request_service::approve_request(&pool, id, approver_member_id).await
}

#[tauri::command]
pub async fn reject_request(
    id: i64,
    reason: String,
    rejector_member_id: i64,
    pool: State<'_, SqlitePool>,
) -> Result<Request, AppError> {
    request_service::reject_request(&pool, id, &reason, rejector_member_id).await
}

#[tauri::command]
pub async fn cancel_request(id: i64, pool: State<'_, SqlitePool>) -> Result<Request, AppError> {
    request_service::cancel_request(&pool, id).await
}

#[tauri::command]
pub async fn list_requests(
    family_id: i64,
    status: Option<String>,
    pool: State<'_, SqlitePool>,
) -> Result<Vec<Request>, AppError> {
    request_service::list_requests(&pool, family_id, status.as_deref()).await
}

#[tauri::command]
pub async fn get_request(id: i64, pool: State<'_, SqlitePool>) -> Result<Request, AppError> {
    request_service::get_request(&pool, id).await
}

#[tauri::command]
pub async fn list_wallet_transactions(
    wallet_id: i64,
    limit: Option<i64>,
    pool: State<'_, SqlitePool>,
) -> Result<Vec<Transaction>, AppError> {
    transaction_service::list_wallet_transactions(&pool, wallet_id, limit).await
}
