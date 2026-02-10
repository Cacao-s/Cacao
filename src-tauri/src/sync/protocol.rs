use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SyncHandshakeRequest {
    pub device_uuid: String,
    pub family_uuid: String,
    pub pairing_code: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SyncHandshakeResponse {
    pub success: bool,
    pub family_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SyncPushRequest {
    pub device_uuid: String,
    pub changes: Vec<SyncChange>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SyncPushResponse {
    pub accepted: Vec<String>,
    pub rejected: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SyncChange {
    pub table: String,
    pub uuid: String,
    pub sync_version: i64,
    pub data: serde_json::Value,
    pub is_deleted: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SyncPullResponse {
    pub changes: Vec<SyncChange>,
    pub server_timestamp: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SyncStatus {
    pub state: String,
    pub last_sync_at: Option<String>,
    pub peer_device_name: Option<String>,
    pub error_message: Option<String>,
}

impl Default for SyncStatus {
    fn default() -> Self {
        Self {
            state: "disconnected".to_string(),
            last_sync_at: None,
            peer_device_name: None,
            error_message: None,
        }
    }
}
