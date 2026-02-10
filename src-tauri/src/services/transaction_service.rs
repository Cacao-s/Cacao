use crate::error::AppError;
use crate::models::params::{ManualTransactionParams, TransactionFilters};
use crate::models::transaction::Transaction;
use serde::Serialize;
use sqlx::SqlitePool;

#[derive(Debug, Clone, Serialize)]
pub struct MonthlySummary {
    pub total_credit_cents: i64,
    pub total_debit_cents: i64,
    pub net_change_cents: i64,
    pub transaction_count: i64,
}

pub async fn list_wallet_transactions(
    pool: &SqlitePool,
    wallet_id: i64,
    limit: Option<i64>,
) -> Result<Vec<Transaction>, AppError> {
    let limit = limit.unwrap_or(50);
    let transactions = sqlx::query_as::<_, Transaction>(
        "SELECT * FROM transactions WHERE wallet_id = ? AND is_deleted = 0 ORDER BY occurred_at DESC LIMIT ?"
    )
    .bind(wallet_id)
    .bind(limit)
    .fetch_all(pool).await?;
    Ok(transactions)
}

pub async fn list_transactions(
    pool: &SqlitePool,
    family_id: i64,
    filters: &TransactionFilters,
) -> Result<Vec<Transaction>, AppError> {
    let mut query =
        String::from("SELECT * FROM transactions WHERE family_id = ? AND is_deleted = 0");
    let mut conditions = Vec::new();

    // Build dynamic WHERE clause
    if filters.date_from.is_some() {
        conditions.push("occurred_at >= ?");
    }
    if filters.date_to.is_some() {
        conditions.push("occurred_at <= ?");
    }
    if filters.wallet_id.is_some() {
        conditions.push("wallet_id = ?");
    }
    if filters.transaction_type.is_some() {
        conditions.push("type = ?");
    }
    if filters.source_type.is_some() {
        conditions.push("source_type = ?");
    }

    for condition in conditions {
        query.push_str(" AND ");
        query.push_str(condition);
    }

    query.push_str(" ORDER BY occurred_at DESC");

    let limit = filters.limit.unwrap_or(50);
    let offset = filters.offset.unwrap_or(0);
    query.push_str(&format!(" LIMIT {} OFFSET {}", limit, offset));

    // Build the query with bindings
    let mut sql_query = sqlx::query_as::<_, Transaction>(&query).bind(family_id);

    if let Some(ref date_from) = filters.date_from {
        sql_query = sql_query.bind(date_from);
    }
    if let Some(ref date_to) = filters.date_to {
        sql_query = sql_query.bind(date_to);
    }
    if let Some(wallet_id) = filters.wallet_id {
        sql_query = sql_query.bind(wallet_id);
    }
    if let Some(ref transaction_type) = filters.transaction_type {
        sql_query = sql_query.bind(transaction_type);
    }
    if let Some(ref source_type) = filters.source_type {
        sql_query = sql_query.bind(source_type);
    }

    let transactions = sql_query.fetch_all(pool).await?;
    Ok(transactions)
}

pub async fn create_manual_transaction(
    pool: &SqlitePool,
    params: &ManualTransactionParams,
) -> Result<Transaction, AppError> {
    // Validate amount
    if params.amount_cents <= 0 {
        return Err(AppError::validation("Amount must be greater than 0"));
    }

    // Validate transaction type
    let transaction_type = params.transaction_type.as_str();
    if transaction_type != "credit" && transaction_type != "debit" {
        return Err(AppError::validation(
            "Transaction type must be 'credit' or 'debit'",
        ));
    }

    // Start SQLite transaction
    let mut tx = pool.begin().await?;

    // Get current wallet balance
    let (current_balance,): (i64,) =
        sqlx::query_as("SELECT balance_cents FROM wallets WHERE id = ? AND is_deleted = 0")
            .bind(params.wallet_id)
            .fetch_optional(&mut *tx)
            .await?
            .ok_or_else(|| AppError::not_found("wallet"))?;

    // Calculate new balance
    let new_balance = if transaction_type == "credit" {
        current_balance + params.amount_cents
    } else {
        current_balance - params.amount_cents
    };

    // Check for negative balance on debit
    if transaction_type == "debit" && new_balance < 0 {
        return Err(AppError::validation("Insufficient balance"));
    }

    // Create transaction record
    let transaction = sqlx::query_as::<_, Transaction>(
        "INSERT INTO transactions (family_id, wallet_id, type, amount_cents, source_type, category, occurred_at, notes) \
         VALUES (?, ?, ?, ?, 'manual', ?, datetime('now'), ?) RETURNING *"
    )
    .bind(params.family_id)
    .bind(params.wallet_id)
    .bind(transaction_type)
    .bind(params.amount_cents)
    .bind(&params.category)
    .bind(&params.notes)
    .fetch_one(&mut *tx).await?;

    // Update wallet balance and increment sync_version
    sqlx::query(
        "UPDATE wallets SET balance_cents = ?, sync_version = sync_version + 1, updated_at = datetime('now') \
         WHERE id = ?"
    )
    .bind(new_balance)
    .bind(params.wallet_id)
    .execute(&mut *tx).await?;

    // Commit transaction
    tx.commit().await?;

    Ok(transaction)
}

pub async fn get_monthly_summary(
    pool: &SqlitePool,
    family_id: i64,
    year: i32,
    month: u32,
) -> Result<MonthlySummary, AppError> {
    // Validate month
    if !(1..=12).contains(&month) {
        return Err(AppError::validation("Month must be between 1 and 12"));
    }

    // Build date range strings
    let start_date = format!("{:04}-{:02}-01 00:00:00", year, month);
    let end_date = if month == 12 {
        format!("{:04}-01-01 00:00:00", year + 1)
    } else {
        format!("{:04}-{:02}-01 00:00:00", year, month + 1)
    };

    // Query for credit total
    let (total_credit,): (Option<i64>,) = sqlx::query_as(
        "SELECT COALESCE(SUM(amount_cents), 0) FROM transactions \
         WHERE family_id = ? AND type = 'credit' AND is_deleted = 0 \
         AND occurred_at >= ? AND occurred_at < ?",
    )
    .bind(family_id)
    .bind(&start_date)
    .bind(&end_date)
    .fetch_one(pool)
    .await?;

    // Query for debit total
    let (total_debit,): (Option<i64>,) = sqlx::query_as(
        "SELECT COALESCE(SUM(amount_cents), 0) FROM transactions \
         WHERE family_id = ? AND type = 'debit' AND is_deleted = 0 \
         AND occurred_at >= ? AND occurred_at < ?",
    )
    .bind(family_id)
    .bind(&start_date)
    .bind(&end_date)
    .fetch_one(pool)
    .await?;

    // Query for transaction count
    let (count,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM transactions \
         WHERE family_id = ? AND is_deleted = 0 \
         AND occurred_at >= ? AND occurred_at < ?",
    )
    .bind(family_id)
    .bind(&start_date)
    .bind(&end_date)
    .fetch_one(pool)
    .await?;

    let total_credit_cents = total_credit.unwrap_or(0);
    let total_debit_cents = total_debit.unwrap_or(0);
    let net_change_cents = total_credit_cents - total_debit_cents;

    Ok(MonthlySummary {
        total_credit_cents,
        total_debit_cents,
        net_change_cents,
        transaction_count: count,
    })
}
