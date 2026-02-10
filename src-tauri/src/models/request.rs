use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum RequestStatus {
    Draft,
    Pending,
    Approved,
    Rejected,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum RequestCategory {
    Food,
    Transport,
    Education,
    Entertainment,
    Clothing,
    Health,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Request {
    pub id: i64,
    pub uuid: String,
    pub family_id: i64,
    pub requester_member_id: i64,
    pub wallet_id: i64,
    pub amount_cents: i64,
    pub category: Option<String>,
    pub notes: Option<String>,
    pub attachment_url: Option<String>,
    pub status: String,
    pub decision_by_member_id: Option<i64>,
    pub decision_at: Option<String>,
    pub rejection_reason: Option<String>,
    pub sync_version: i64,
    pub last_synced_at: Option<String>,
    pub is_deleted: i64,
    pub created_at: String,
    pub updated_at: String,
}
