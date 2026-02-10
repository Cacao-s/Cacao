use crate::error::AppError;
use crate::models::allowance::Allowance;
use crate::models::params::{CreateAllowanceParams, UpdateAllowanceParams};
use crate::services::allowance_service;
use sqlx::SqlitePool;
use tauri::State;

#[tauri::command]
pub async fn create_allowance(
    params: CreateAllowanceParams,
    pool: State<'_, SqlitePool>,
) -> Result<Allowance, AppError> {
    allowance_service::create_allowance(&pool, &params).await
}

#[tauri::command]
pub async fn update_allowance(
    id: i64,
    params: UpdateAllowanceParams,
    pool: State<'_, SqlitePool>,
) -> Result<Allowance, AppError> {
    allowance_service::update_allowance(&pool, id, &params).await
}

#[tauri::command]
pub async fn pause_allowance(id: i64, pool: State<'_, SqlitePool>) -> Result<(), AppError> {
    allowance_service::pause_allowance(&pool, id).await
}

#[tauri::command]
pub async fn resume_allowance(id: i64, pool: State<'_, SqlitePool>) -> Result<(), AppError> {
    allowance_service::resume_allowance(&pool, id).await
}

#[tauri::command]
pub async fn list_allowances(
    family_id: i64,
    pool: State<'_, SqlitePool>,
) -> Result<Vec<Allowance>, AppError> {
    allowance_service::list_allowances(&pool, family_id).await
}
