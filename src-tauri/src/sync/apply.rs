use crate::sync::protocol::SyncChange;
use sqlx::SqlitePool;

/// Baby-side: apply Giver's changes (Giver is authoritative, always overwrite)
pub async fn apply_changes(
    pool: &SqlitePool,
    changes: Vec<SyncChange>,
) -> Result<Vec<i64>, String> {
    let mut affected_wallet_ids = Vec::new();

    for change in &changes {
        if change.table == "wallets" || change.table == "transactions" {
            if let Some(wid) = change.data.get("wallet_id").and_then(|v| v.as_i64())
                && !affected_wallet_ids.contains(&wid)
            {
                affected_wallet_ids.push(wid);
            }
            if change.table == "wallets"
                && let Some(wid) = change.data.get("id").and_then(|v| v.as_i64())
                && !affected_wallet_ids.contains(&wid)
            {
                affected_wallet_ids.push(wid);
            }
        }

        upsert_from_giver(pool, change).await?;
    }

    // Recalculate balances for affected wallets
    for wallet_id in &affected_wallet_ids {
        recalculate_balance(pool, *wallet_id).await?;
    }

    // Update last sync timestamp
    let timestamp = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    sqlx::query("INSERT OR REPLACE INTO sync_state (key, value) VALUES ('last_sync_timestamp', ?)")
        .bind(&timestamp)
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(affected_wallet_ids)
}

async fn upsert_from_giver(pool: &SqlitePool, change: &SyncChange) -> Result<(), String> {
    let data = change.data.as_object().ok_or("Invalid change data")?;

    let columns: Vec<&str> = data.keys().map(|k| k.as_str()).collect();
    let placeholders: Vec<String> = (0..columns.len()).map(|_| "?".to_string()).collect();

    // Use INSERT OR REPLACE for Giver-authoritative upsert
    let query = format!(
        "INSERT OR REPLACE INTO {} ({}) VALUES ({})",
        change.table,
        columns.join(", "),
        placeholders.join(", ")
    );

    let mut q = sqlx::query(&query);
    for col in &columns {
        let value = data.get(*col).unwrap_or(&serde_json::Value::Null);
        q = bind_json_value(q, value);
    }

    q.execute(pool)
        .await
        .map_err(|e| format!("Upsert error for {}: {}", change.table, e))?;
    Ok(())
}

async fn recalculate_balance(pool: &SqlitePool, wallet_id: i64) -> Result<(), String> {
    let balance: Option<i64> = sqlx::query_scalar(
        "SELECT SUM(CASE WHEN type='credit' THEN amount_cents ELSE -amount_cents END) \
         FROM transactions WHERE wallet_id = ? AND is_deleted = 0",
    )
    .bind(wallet_id)
    .fetch_one(pool)
    .await
    .map_err(|e| e.to_string())?;

    let balance = balance.unwrap_or(0);

    sqlx::query("UPDATE wallets SET balance_cents = ? WHERE id = ?")
        .bind(balance)
        .bind(wallet_id)
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

fn bind_json_value<'q>(
    query: sqlx::query::Query<'q, sqlx::Sqlite, sqlx::sqlite::SqliteArguments<'q>>,
    value: &'q serde_json::Value,
) -> sqlx::query::Query<'q, sqlx::Sqlite, sqlx::sqlite::SqliteArguments<'q>> {
    match value {
        serde_json::Value::Null => query.bind(None::<String>),
        serde_json::Value::Bool(b) => query.bind(*b as i64),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                query.bind(i)
            } else if let Some(f) = n.as_f64() {
                query.bind(f)
            } else {
                query.bind(n.to_string())
            }
        }
        serde_json::Value::String(s) => query.bind(s.as_str()),
        _ => query.bind(value.to_string()),
    }
}
