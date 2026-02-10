use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    Giver,
    Baby,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Profile {
    pub id: i64,
    pub uuid: String,
    pub display_name: String,
    pub role: String,
    pub locale: String,
    pub theme: String,
    pub pin_hash: Option<String>,
    pub sync_version: i64,
    pub last_synced_at: Option<String>,
    pub is_deleted: i64,
    pub created_at: String,
    pub updated_at: String,
}
