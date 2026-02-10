use crate::error::AppError;
use crate::models::family::{Family, FamilyMember};
use crate::models::profile::Profile;
use crate::services::{family_service, pairing_service, setup_service};
use sqlx::SqlitePool;
use tauri::State;

#[tauri::command]
pub async fn setup_device(
    display_name: String,
    role: String,
    pool: State<'_, SqlitePool>,
) -> Result<Profile, AppError> {
    setup_service::setup_device(&pool, &display_name, &role).await
}

#[tauri::command]
pub async fn get_profile(pool: State<'_, SqlitePool>) -> Result<Option<Profile>, AppError> {
    setup_service::get_profile(&pool).await
}

#[tauri::command]
pub async fn is_device_setup(pool: State<'_, SqlitePool>) -> Result<bool, AppError> {
    setup_service::is_setup(&pool).await
}

#[tauri::command]
pub async fn create_family(name: String, pool: State<'_, SqlitePool>) -> Result<Family, AppError> {
    // Get the current profile to use its uuid as the device identifier
    let profile = setup_service::get_profile(&pool).await?.ok_or_else(|| {
        AppError::new(
            "DEVICE_NOT_SETUP",
            "Device must be set up before creating a family",
        )
    })?;
    family_service::create_family(&pool, &profile.uuid, &name).await
}

#[tauri::command]
pub async fn get_family(pool: State<'_, SqlitePool>) -> Result<Option<Family>, AppError> {
    family_service::get_family(&pool).await
}

#[tauri::command]
pub async fn get_family_members(
    family_id: i64,
    pool: State<'_, SqlitePool>,
) -> Result<Vec<FamilyMember>, AppError> {
    family_service::get_members(&pool, family_id).await
}

#[tauri::command]
pub async fn generate_pairing_code(
    family_id: i64,
    pool: State<'_, SqlitePool>,
) -> Result<String, AppError> {
    pairing_service::generate_pairing_code(&pool, family_id).await
}

#[tauri::command]
pub async fn join_family_with_code(
    code: String,
    pool: State<'_, SqlitePool>,
) -> Result<Family, AppError> {
    // Validate the pairing code and get the family_id
    let family_id = pairing_service::validate_pairing_code(&pool, &code).await?;

    // Get the current profile
    let profile = setup_service::get_profile(&pool).await?.ok_or_else(|| {
        AppError::new(
            "DEVICE_NOT_SETUP",
            "Device must be set up before joining a family",
        )
    })?;

    // Add this device as a member of the family
    family_service::add_member(&pool, family_id, &profile.uuid, &profile.role).await?;

    // Return the family
    let family = family_service::get_family(&pool)
        .await?
        .ok_or_else(|| AppError::not_found("family"))?;
    Ok(family)
}

#[tauri::command]
pub async fn remove_family_member(
    member_id: i64,
    pool: State<'_, SqlitePool>,
) -> Result<(), AppError> {
    family_service::remove_member(&pool, member_id).await
}

#[tauri::command]
pub async fn reset_device(pool: State<'_, SqlitePool>) -> Result<(), AppError> {
    setup_service::reset_device(&pool).await
}
