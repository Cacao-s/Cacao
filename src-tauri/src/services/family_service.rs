use crate::error::AppError;
use crate::models::family::{Family, FamilyMember};
use sqlx::SqlitePool;

pub async fn create_family(
    pool: &SqlitePool,
    device_uuid: &str,
    name: &str,
) -> Result<Family, AppError> {
    let name = name.trim();
    if name.is_empty() || name.len() > 50 {
        return Err(AppError::validation("Family name must be 1-50 characters"));
    }

    let family = sqlx::query_as::<_, Family>(
        "INSERT INTO families (name, created_by_device) VALUES (?, ?) RETURNING *",
    )
    .bind(name)
    .bind(device_uuid)
    .fetch_one(pool)
    .await?;

    // Auto-create family member for the creator (giver)
    sqlx::query(
        "INSERT INTO family_members (family_id, profile_uuid, family_role, joined_at) VALUES (?, ?, 'giver', datetime('now'))"
    )
    .bind(family.id)
    .bind(device_uuid)
    .execute(pool).await?;

    Ok(family)
}

pub async fn get_family(pool: &SqlitePool) -> Result<Option<Family>, AppError> {
    let family = sqlx::query_as::<_, Family>("SELECT * FROM families WHERE is_deleted = 0 LIMIT 1")
        .fetch_optional(pool)
        .await?;
    Ok(family)
}

pub async fn get_members(pool: &SqlitePool, family_id: i64) -> Result<Vec<FamilyMember>, AppError> {
    let members = sqlx::query_as::<_, FamilyMember>(
        "SELECT * FROM family_members WHERE family_id = ? AND is_deleted = 0",
    )
    .bind(family_id)
    .fetch_all(pool)
    .await?;
    Ok(members)
}

pub async fn add_member(
    pool: &SqlitePool,
    family_id: i64,
    profile_uuid: &str,
    family_role: &str,
) -> Result<FamilyMember, AppError> {
    let member = sqlx::query_as::<_, FamilyMember>(
        "INSERT INTO family_members (family_id, profile_uuid, family_role, joined_at) VALUES (?, ?, ?, datetime('now')) RETURNING *"
    )
    .bind(family_id)
    .bind(profile_uuid)
    .bind(family_role)
    .fetch_one(pool).await?;
    Ok(member)
}

pub async fn remove_member(pool: &SqlitePool, member_id: i64) -> Result<(), AppError> {
    sqlx::query(
        "UPDATE family_members SET status = 'removed', is_deleted = 1, sync_version = sync_version + 1, updated_at = datetime('now') WHERE id = ?"
    )
    .bind(member_id)
    .execute(pool).await?;
    Ok(())
}
