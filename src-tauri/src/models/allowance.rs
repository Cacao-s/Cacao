use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Frequency {
    Daily,
    Weekly,
    Biweekly,
    Monthly,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AllowanceStatus {
    Active,
    Paused,
    Archived,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Allowance {
    pub id: i64,
    pub uuid: String,
    pub family_id: i64,
    pub giver_member_id: i64,
    pub receiver_member_id: i64,
    pub wallet_id: i64,
    pub amount_cents: i64,
    pub frequency: String,
    pub interval_count: i64,
    pub next_run_at: Option<String>,
    pub last_run_at: Option<String>,
    pub status: String,
    pub notes: Option<String>,
    pub sync_version: i64,
    pub last_synced_at: Option<String>,
    pub is_deleted: i64,
    pub created_at: String,
    pub updated_at: String,
}
