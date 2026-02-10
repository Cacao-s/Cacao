use crate::error::AppError;
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};
use sqlx::SqlitePool;

pub async fn set_pin(pool: &SqlitePool, pin: &str) -> Result<(), AppError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let pin_hash = argon2
        .hash_password(pin.as_bytes(), &salt)
        .map_err(|e| AppError::internal(format!("Failed to hash PIN: {}", e)))?
        .to_string();

    sqlx::query("UPDATE profiles SET pin_hash = ? WHERE is_deleted = 0")
        .bind(pin_hash)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn verify_pin(pool: &SqlitePool, pin: &str) -> Result<bool, AppError> {
    let result = sqlx::query_scalar::<_, Option<String>>(
        "SELECT pin_hash FROM profiles WHERE is_deleted = 0 LIMIT 1",
    )
    .fetch_optional(pool)
    .await?;

    match result.flatten() {
        Some(hash) => {
            let parsed_hash = PasswordHash::new(&hash)
                .map_err(|e| AppError::internal(format!("Failed to parse stored hash: {}", e)))?;

            let argon2 = Argon2::default();
            Ok(argon2.verify_password(pin.as_bytes(), &parsed_hash).is_ok())
        }
        None => Ok(false),
    }
}

pub async fn remove_pin(pool: &SqlitePool) -> Result<(), AppError> {
    sqlx::query("UPDATE profiles SET pin_hash = NULL WHERE is_deleted = 0")
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn has_pin(pool: &SqlitePool) -> Result<bool, AppError> {
    let result = sqlx::query_scalar::<_, Option<String>>(
        "SELECT pin_hash FROM profiles WHERE is_deleted = 0 LIMIT 1",
    )
    .fetch_optional(pool)
    .await?;

    Ok(result.flatten().is_some())
}
