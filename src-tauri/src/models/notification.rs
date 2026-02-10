use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum EventType {
    RequestSubmitted,
    RequestApproved,
    RequestRejected,
    AllowanceDisbursed,
    LowBalance,
    MemberJoined,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Notification {
    pub id: i64,
    pub event_type: String,
    pub payload: Option<String>,
    pub is_read: i64,
    pub read_at: Option<String>,
    pub created_at: String,
}
