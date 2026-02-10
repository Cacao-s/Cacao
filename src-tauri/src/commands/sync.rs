use crate::error::AppError;
use crate::sync::coordinator::SyncCoordinator;
use crate::sync::protocol::SyncStatus;
use std::sync::Arc;
use tauri::State;
use tokio::sync::Mutex;

#[tauri::command]
pub async fn get_sync_status(
    coordinator: State<'_, Arc<Mutex<SyncCoordinator>>>,
) -> Result<SyncStatus, AppError> {
    let coord = coordinator.lock().await;
    Ok(coord.status().await)
}

#[tauri::command]
pub async fn trigger_sync(
    coordinator: State<'_, Arc<Mutex<SyncCoordinator>>>,
) -> Result<(), AppError> {
    let coord = coordinator.lock().await;
    coord
        .sync_now()
        .await
        .map_err(|e| AppError::new("SYNC_ERROR", e))
}

#[tauri::command]
pub async fn stop_sync(
    coordinator: State<'_, Arc<Mutex<SyncCoordinator>>>,
) -> Result<(), AppError> {
    let mut coord = coordinator.lock().await;
    coord.stop().await;
    Ok(())
}
