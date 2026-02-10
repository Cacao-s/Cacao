use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct CreateWalletParams {
    pub family_id: i64,
    pub name: String,
    pub wallet_type: String,
    pub initial_balance_cents: Option<i64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateRequestParams {
    pub family_id: i64,
    pub requester_member_id: i64,
    pub wallet_id: i64,
    pub amount_cents: i64,
    pub category: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateRequestParams {
    pub amount_cents: Option<i64>,
    pub category: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateAllowanceParams {
    pub family_id: i64,
    pub giver_member_id: i64,
    pub receiver_member_id: i64,
    pub wallet_id: i64,
    pub amount_cents: i64,
    pub frequency: String,
    pub interval_count: Option<i64>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateAllowanceParams {
    pub amount_cents: Option<i64>,
    pub frequency: Option<String>,
    pub interval_count: Option<i64>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ManualTransactionParams {
    pub family_id: i64,
    pub wallet_id: i64,
    pub transaction_type: String,
    pub amount_cents: i64,
    pub category: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TransactionFilters {
    pub date_from: Option<String>,
    pub date_to: Option<String>,
    pub wallet_id: Option<i64>,
    pub transaction_type: Option<String>,
    pub source_type: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}
