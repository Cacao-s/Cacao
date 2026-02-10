use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum FamilyRole {
    Giver,
    Baby,
    Viewer,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum MemberStatus {
    Active,
    Removed,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Family {
    pub id: i64,
    pub uuid: String,
    pub name: String,
    pub currency: String,
    pub timezone: String,
    pub created_by_device: String,
    pub sync_version: i64,
    pub last_synced_at: Option<String>,
    pub is_deleted: i64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct FamilyMember {
    pub id: i64,
    pub uuid: String,
    pub family_id: i64,
    pub profile_uuid: String,
    pub family_role: String,
    pub status: String,
    pub joined_at: Option<String>,
    pub sync_version: i64,
    pub last_synced_at: Option<String>,
    pub is_deleted: i64,
    pub created_at: String,
    pub updated_at: String,
}
