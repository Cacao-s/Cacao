use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct PairedDevice {
    pub id: i64,
    pub device_uuid: String,
    pub display_name: String,
    pub role: String,
    pub family_id: i64,
    pub last_seen_at: Option<String>,
    pub last_sync_at: Option<String>,
    pub is_active: i64,
    pub created_at: String,
}
