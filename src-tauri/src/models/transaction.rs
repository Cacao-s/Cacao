use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TransactionType {
    Credit,
    Debit,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SourceType {
    Allowance,
    Request,
    Manual,
    Adjustment,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Transaction {
    pub id: i64,
    pub uuid: String,
    pub family_id: i64,
    pub wallet_id: i64,
    #[serde(rename = "type")]
    #[sqlx(rename = "type")]
    pub type_: String,
    pub amount_cents: i64,
    pub source_type: String,
    pub source_id: Option<i64>,
    pub category: Option<String>,
    pub occurred_at: String,
    pub notes: Option<String>,
    pub sync_version: i64,
    pub last_synced_at: Option<String>,
    pub is_deleted: i64,
    pub created_at: String,
}
