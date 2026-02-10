use crate::sync::protocol::SyncChange;
use sqlx::{Row, SqlitePool};

pub struct MergeResult {
    pub accepted: Vec<String>,
    pub rejected: Vec<String>,
}

const SYNCABLE_TABLES: &[&str] = &[
    "profiles",
    "families",
    "family_members",
    "wallets",
    "allowances",
    "requests",
    "transactions",
];

/// Giver-side merge: process Baby's pushed changes
pub async fn merge_changes(
    pool: &SqlitePool,
    changes: Vec<SyncChange>,
) -> Result<MergeResult, String> {
    let mut accepted = Vec::new();
    let mut rejected = Vec::new();

    for change in changes {
        if !SYNCABLE_TABLES.contains(&change.table.as_str()) {
            rejected.push(change.uuid.clone());
            continue;
        }

        match change.table.as_str() {
            "transactions" => {
                // Append-only: accept if uuid doesn't exist
                let exists = record_exists(pool, "transactions", &change.uuid)
                    .await
                    .map_err(|e| e.to_string())?;
                if !exists {
                    insert_record(pool, &change)
                        .await
                        .map_err(|e| e.to_string())?;
                    accepted.push(change.uuid);
                }
                // Already exists: silently ignore (not a rejection)
            }
            table => {
                let local_version = get_record_version(pool, table, &change.uuid)
                    .await
                    .map_err(|e| e.to_string())?;

                match local_version {
                    Some(local_v) => {
                        if change.sync_version > local_v {
                            upsert_record(pool, &change)
                                .await
                                .map_err(|e| e.to_string())?;
                            accepted.push(change.uuid);
                        } else {
                            // Giver version >= Baby: Giver wins
                            rejected.push(change.uuid);
                        }
                    }
                    None => {
                        insert_record(pool, &change)
                            .await
                            .map_err(|e| e.to_string())?;
                        accepted.push(change.uuid);
                    }
                }
            }
        }
    }

    Ok(MergeResult { accepted, rejected })
}

/// Collect all changes since a given timestamp for pull
pub async fn collect_changes_since(
    pool: &SqlitePool,
    since: &str,
) -> Result<Vec<SyncChange>, String> {
    let mut changes = Vec::new();

    for &table in SYNCABLE_TABLES {
        // transactions only has created_at, no updated_at
        let query = if table == "transactions" {
            format!("SELECT * FROM {} WHERE created_at > ?", table)
        } else {
            format!(
                "SELECT * FROM {} WHERE updated_at > ? OR created_at > ?",
                table
            )
        };

        let rows = if table == "transactions" {
            sqlx::query(&query)
                .bind(since)
                .fetch_all(pool)
                .await
                .map_err(|e| e.to_string())?
        } else {
            sqlx::query(&query)
                .bind(since)
                .bind(since)
                .fetch_all(pool)
                .await
                .map_err(|e| e.to_string())?
        };

        for row in rows {
            let uuid: String = row.try_get("uuid").unwrap_or_default();
            let sync_version: i64 = row.try_get("sync_version").unwrap_or(0);
            let is_deleted: i64 = row.try_get("is_deleted").unwrap_or(0);

            // Serialize entire row to JSON
            let data = row_to_json(&row, table);

            changes.push(SyncChange {
                table: table.to_string(),
                uuid,
                sync_version,
                data,
                is_deleted: is_deleted != 0,
            });
        }
    }

    Ok(changes)
}

async fn record_exists(pool: &SqlitePool, table: &str, uuid: &str) -> Result<bool, sqlx::Error> {
    let query = format!("SELECT COUNT(*) as cnt FROM {} WHERE uuid = ?", table);
    let count: i64 = sqlx::query_scalar(&query)
        .bind(uuid)
        .fetch_one(pool)
        .await?;
    Ok(count > 0)
}

async fn get_record_version(
    pool: &SqlitePool,
    table: &str,
    uuid: &str,
) -> Result<Option<i64>, sqlx::Error> {
    let query = format!("SELECT sync_version FROM {} WHERE uuid = ?", table);
    let version: Option<i64> = sqlx::query_scalar(&query)
        .bind(uuid)
        .fetch_optional(pool)
        .await?;
    Ok(version)
}

async fn insert_record(pool: &SqlitePool, change: &SyncChange) -> Result<(), sqlx::Error> {
    let data = change.data.as_object();
    if data.is_none() {
        return Ok(());
    }
    let data = data.unwrap();

    let columns: Vec<&str> = data.keys().map(|k| k.as_str()).collect();
    let placeholders: Vec<String> = (0..columns.len()).map(|_| "?".to_string()).collect();

    let query = format!(
        "INSERT OR IGNORE INTO {} ({}) VALUES ({})",
        change.table,
        columns.join(", "),
        placeholders.join(", ")
    );

    let mut q = sqlx::query(&query);
    for col in &columns {
        q = bind_json_value(q, data.get(*col).unwrap_or(&serde_json::Value::Null));
    }

    q.execute(pool).await?;
    Ok(())
}

async fn upsert_record(pool: &SqlitePool, change: &SyncChange) -> Result<(), sqlx::Error> {
    let data = change.data.as_object();
    if data.is_none() {
        return Ok(());
    }
    let data = data.unwrap();

    // Build SET clause (skip id)
    let set_clauses: Vec<String> = data
        .keys()
        .filter(|k| k.as_str() != "id")
        .map(|k| format!("{} = ?", k))
        .collect();

    if set_clauses.is_empty() {
        return Ok(());
    }

    let query = format!(
        "UPDATE {} SET {} WHERE uuid = ?",
        change.table,
        set_clauses.join(", ")
    );

    let mut q = sqlx::query(&query);
    for (key, value) in data {
        if key == "id" {
            continue;
        }
        q = bind_json_value(q, value);
    }
    q = q.bind(&change.uuid);

    q.execute(pool).await?;
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

fn row_to_json(row: &sqlx::sqlite::SqliteRow, _table: &str) -> serde_json::Value {
    use sqlx::Column;
    let mut map = serde_json::Map::new();
    for col in row.columns() {
        let name = col.name();
        // Try to get as different types
        if let Ok(v) = row.try_get::<i64, _>(name) {
            map.insert(name.to_string(), serde_json::Value::Number(v.into()));
        } else if let Ok(v) = row.try_get::<f64, _>(name) {
            if let Some(n) = serde_json::Number::from_f64(v) {
                map.insert(name.to_string(), serde_json::Value::Number(n));
            }
        } else if let Ok(v) = row.try_get::<String, _>(name) {
            map.insert(name.to_string(), serde_json::Value::String(v));
        } else if let Ok(v) = row.try_get::<Option<String>, _>(name) {
            match v {
                Some(s) => map.insert(name.to_string(), serde_json::Value::String(s)),
                None => map.insert(name.to_string(), serde_json::Value::Null),
            };
        } else {
            map.insert(name.to_string(), serde_json::Value::Null);
        }
    }
    serde_json::Value::Object(map)
}
