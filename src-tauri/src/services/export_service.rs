use crate::error::AppError;
use sqlx::SqlitePool;

#[derive(sqlx::FromRow)]
struct TransactionExport {
    occurred_at: String,
    wallet_name: String,
    type_: String,
    amount_cents: i64,
    source_type: String,
    category: Option<String>,
    notes: Option<String>,
}

pub async fn export_transactions_csv(
    pool: &SqlitePool,
    family_id: i64,
    date_from: Option<String>,
    date_to: Option<String>,
    wallet_id: Option<i64>,
) -> Result<String, AppError> {
    let mut query = String::from(
        "SELECT t.occurred_at, w.name as wallet_name, t.type as type_, t.amount_cents, \
         t.source_type, t.category, t.notes \
         FROM transactions t \
         JOIN wallets w ON t.wallet_id = w.id \
         WHERE t.family_id = ? AND t.is_deleted = 0",
    );

    if date_from.is_some() {
        query.push_str(" AND t.occurred_at >= ?");
    }
    if date_to.is_some() {
        query.push_str(" AND t.occurred_at <= ?");
    }
    if wallet_id.is_some() {
        query.push_str(" AND t.wallet_id = ?");
    }

    query.push_str(" ORDER BY t.occurred_at DESC");

    let mut sql_query = sqlx::query_as::<_, TransactionExport>(&query).bind(family_id);

    if let Some(from) = date_from {
        sql_query = sql_query.bind(from);
    }
    if let Some(to) = date_to {
        sql_query = sql_query.bind(to);
    }
    if let Some(wid) = wallet_id {
        sql_query = sql_query.bind(wid);
    }

    let transactions = sql_query.fetch_all(pool).await?;

    let mut csv = String::from("Date,Wallet,Type,Amount,Source,Category,Notes\n");

    for tx in transactions {
        let amount = (tx.amount_cents as f64) / 100.0;
        csv.push_str(&format!(
            "{},{},{},{:.2},{},{},{}\n",
            tx.occurred_at,
            escape_csv_field(&tx.wallet_name),
            tx.type_,
            amount,
            tx.source_type,
            escape_csv_field(&tx.category.unwrap_or_default()),
            escape_csv_field(&tx.notes.unwrap_or_default())
        ));
    }

    Ok(csv)
}

fn escape_csv_field(field: &str) -> String {
    if field.contains(',') || field.contains('"') || field.contains('\n') {
        format!("\"{}\"", field.replace('"', "\"\""))
    } else {
        field.to_string()
    }
}
