use crate::error::AppError;
use crate::models::allowance::Allowance;
use crate::models::params::{CreateAllowanceParams, UpdateAllowanceParams};
use chrono::{DateTime, Datelike, Duration, Utc};
use sqlx::SqlitePool;

/// Calculate the next run date based on frequency and interval
fn calculate_next_run(
    frequency: &str,
    interval: i64,
    from: DateTime<Utc>,
) -> Result<DateTime<Utc>, AppError> {
    match frequency {
        "daily" => Ok(from + Duration::days(interval)),
        "weekly" => Ok(from + Duration::weeks(interval)),
        "biweekly" => Ok(from + Duration::weeks(2 * interval)),
        "monthly" => {
            // Add months by incrementing the month field
            let mut year = from.year();
            let mut month = from.month() as i32;
            month += interval as i32;
            while month > 12 {
                month -= 12;
                year += 1;
            }
            while month < 1 {
                month += 12;
                year -= 1;
            }

            // Handle day overflow (e.g., Jan 31 -> Feb 28/29)
            let day = from.day();
            let days_in_month = match month {
                2 => {
                    if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) {
                        29
                    } else {
                        28
                    }
                }
                4 | 6 | 9 | 11 => 30,
                _ => 31,
            };
            let clamped_day = day.min(days_in_month);

            from.with_year(year)
                .and_then(|dt| dt.with_month(month as u32))
                .and_then(|dt| dt.with_day(clamped_day))
                .ok_or_else(|| AppError::validation("Invalid date calculation"))
        }
        "custom" => Ok(from + Duration::days(interval)),
        _ => Err(AppError::validation("Invalid frequency")),
    }
}

pub async fn create_allowance(
    pool: &SqlitePool,
    params: &CreateAllowanceParams,
) -> Result<Allowance, AppError> {
    // Validate amount
    if params.amount_cents <= 0 {
        return Err(AppError::validation("Amount must be greater than 0"));
    }

    // Validate frequency
    let frequency = params.frequency.as_str();
    if !["daily", "weekly", "biweekly", "monthly", "custom"].contains(&frequency) {
        return Err(AppError::validation("Invalid frequency"));
    }

    let interval_count = params.interval_count.unwrap_or(1);
    if interval_count <= 0 {
        return Err(AppError::validation(
            "Interval count must be greater than 0",
        ));
    }

    // Calculate next run at
    let now = Utc::now();
    let next_run_at = calculate_next_run(frequency, interval_count, now)?;
    let next_run_str = next_run_at.format("%Y-%m-%d %H:%M:%S").to_string();

    let allowance = sqlx::query_as::<_, Allowance>(
        "INSERT INTO allowances (family_id, giver_member_id, receiver_member_id, wallet_id, amount_cents, frequency, interval_count, next_run_at, status, notes) \
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, 'active', ?) RETURNING *"
    )
    .bind(params.family_id)
    .bind(params.giver_member_id)
    .bind(params.receiver_member_id)
    .bind(params.wallet_id)
    .bind(params.amount_cents)
    .bind(frequency)
    .bind(interval_count)
    .bind(&next_run_str)
    .bind(&params.notes)
    .fetch_one(pool).await?;

    Ok(allowance)
}

pub async fn update_allowance(
    pool: &SqlitePool,
    id: i64,
    params: &UpdateAllowanceParams,
) -> Result<Allowance, AppError> {
    // Check if allowance exists and is active
    let allowance =
        sqlx::query_as::<_, Allowance>("SELECT * FROM allowances WHERE id = ? AND is_deleted = 0")
            .bind(id)
            .fetch_optional(pool)
            .await?
            .ok_or_else(|| AppError::not_found("allowance"))?;

    if allowance.status != "active" {
        return Err(AppError::validation(
            "Only active allowances can be updated",
        ));
    }

    // Build dynamic update query
    let mut has_updates = false;

    if let Some(amount) = params.amount_cents {
        if amount <= 0 {
            return Err(AppError::validation("Amount must be greater than 0"));
        }
        has_updates = true;
    }

    if let Some(frequency) = &params.frequency {
        if !["daily", "weekly", "biweekly", "monthly", "custom"].contains(&frequency.as_str()) {
            return Err(AppError::validation("Invalid frequency"));
        }
        has_updates = true;
    }

    if let Some(interval) = params.interval_count {
        if interval <= 0 {
            return Err(AppError::validation(
                "Interval count must be greater than 0",
            ));
        }
        has_updates = true;
    }

    if params.notes.is_some() {
        has_updates = true;
    }

    if !has_updates {
        return Ok(allowance);
    }

    // Perform the update using individual fields
    sqlx::query(
        "UPDATE allowances SET
         amount_cents = COALESCE(?, amount_cents),
         frequency = COALESCE(?, frequency),
         interval_count = COALESCE(?, interval_count),
         notes = COALESCE(?, notes),
         sync_version = sync_version + 1,
         updated_at = datetime('now')
         WHERE id = ?",
    )
    .bind(params.amount_cents)
    .bind(&params.frequency)
    .bind(params.interval_count)
    .bind(&params.notes)
    .bind(id)
    .execute(pool)
    .await?;

    // Re-fetch updated allowance
    let updated = sqlx::query_as::<_, Allowance>("SELECT * FROM allowances WHERE id = ?")
        .bind(id)
        .fetch_one(pool)
        .await?;

    Ok(updated)
}

pub async fn pause_allowance(pool: &SqlitePool, id: i64) -> Result<(), AppError> {
    let result = sqlx::query(
        "UPDATE allowances SET status = 'paused', sync_version = sync_version + 1, updated_at = datetime('now') \
         WHERE id = ? AND is_deleted = 0"
    )
    .bind(id)
    .execute(pool).await?;

    if result.rows_affected() == 0 {
        return Err(AppError::not_found("allowance"));
    }

    Ok(())
}

pub async fn resume_allowance(pool: &SqlitePool, id: i64) -> Result<(), AppError> {
    // Get the current allowance
    let allowance =
        sqlx::query_as::<_, Allowance>("SELECT * FROM allowances WHERE id = ? AND is_deleted = 0")
            .bind(id)
            .fetch_optional(pool)
            .await?
            .ok_or_else(|| AppError::not_found("allowance"))?;

    // Recalculate next_run_at from now
    let now = Utc::now();
    let next_run_at = calculate_next_run(&allowance.frequency, allowance.interval_count, now)?;
    let next_run_str = next_run_at.format("%Y-%m-%d %H:%M:%S").to_string();

    sqlx::query(
        "UPDATE allowances SET status = 'active', next_run_at = ?, sync_version = sync_version + 1, updated_at = datetime('now') \
         WHERE id = ?"
    )
    .bind(&next_run_str)
    .bind(id)
    .execute(pool).await?;

    Ok(())
}

pub async fn archive_allowance(pool: &SqlitePool, id: i64) -> Result<(), AppError> {
    let result = sqlx::query(
        "UPDATE allowances SET status = 'archived', sync_version = sync_version + 1, updated_at = datetime('now') \
         WHERE id = ? AND is_deleted = 0"
    )
    .bind(id)
    .execute(pool).await?;

    if result.rows_affected() == 0 {
        return Err(AppError::not_found("allowance"));
    }

    Ok(())
}

pub async fn list_allowances(
    pool: &SqlitePool,
    family_id: i64,
) -> Result<Vec<Allowance>, AppError> {
    let allowances = sqlx::query_as::<_, Allowance>(
        "SELECT * FROM allowances WHERE family_id = ? AND is_deleted = 0 ORDER BY created_at DESC",
    )
    .bind(family_id)
    .fetch_all(pool)
    .await?;

    Ok(allowances)
}

pub async fn execute_allowance(pool: &SqlitePool, allowance: &Allowance) -> Result<(), AppError> {
    // Start a SQLite transaction
    let mut tx = pool.begin().await?;

    // 1. Get the wallet and verify it exists
    let wallet: (i64, i64) =
        sqlx::query_as("SELECT id, balance_cents FROM wallets WHERE id = ? AND is_deleted = 0")
            .bind(allowance.wallet_id)
            .fetch_optional(&mut *tx)
            .await?
            .ok_or_else(|| AppError::not_found("wallet"))?;

    let new_balance = wallet.1 + allowance.amount_cents;

    // 2. Update wallet balance and sync_version
    sqlx::query(
        "UPDATE wallets SET balance_cents = ?, sync_version = sync_version + 1, updated_at = datetime('now') \
         WHERE id = ?"
    )
    .bind(new_balance)
    .bind(allowance.wallet_id)
    .execute(&mut *tx).await?;

    // 3. Create transaction record
    sqlx::query(
        "INSERT INTO transactions (family_id, wallet_id, type, amount_cents, source_type, source_id, occurred_at, notes) \
         VALUES (?, ?, 'credit', ?, 'allowance', ?, datetime('now'), ?)"
    )
    .bind(allowance.family_id)
    .bind(allowance.wallet_id)
    .bind(allowance.amount_cents)
    .bind(allowance.id)
    .bind(format!("Allowance disbursement: {}", allowance.amount_cents))
    .execute(&mut *tx).await?;

    // 4. Calculate next_run_at
    let last_run = if let Some(last) = &allowance.last_run_at {
        DateTime::parse_from_str(last, "%Y-%m-%d %H:%M:%S")
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(|_| Utc::now())
    } else if let Some(next) = &allowance.next_run_at {
        DateTime::parse_from_str(next, "%Y-%m-%d %H:%M:%S")
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(|_| Utc::now())
    } else {
        Utc::now()
    };

    let next_run_at = calculate_next_run(&allowance.frequency, allowance.interval_count, last_run)?;
    let next_run_str = next_run_at.format("%Y-%m-%d %H:%M:%S").to_string();
    let now_str = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

    // 5. Update allowance with last_run_at and next_run_at
    sqlx::query(
        "UPDATE allowances SET last_run_at = ?, next_run_at = ?, sync_version = sync_version + 1, updated_at = datetime('now') \
         WHERE id = ?"
    )
    .bind(&now_str)
    .bind(&next_run_str)
    .bind(allowance.id)
    .execute(&mut *tx).await?;

    // 6. Create notification for allowance disbursement
    let payload = serde_json::json!({
        "allowance_id": allowance.id,
        "wallet_id": allowance.wallet_id,
        "amount_cents": allowance.amount_cents,
        "receiver_member_id": allowance.receiver_member_id
    })
    .to_string();

    sqlx::query(
        "INSERT INTO notifications (event_type, payload, is_read) \
         VALUES ('allowance_disbursed', ?, 0)",
    )
    .bind(&payload)
    .execute(&mut *tx)
    .await?;

    // 7. Create audit log entry
    sqlx::query(
        "INSERT INTO audit_logs (action, entity_type, entity_id, details) \
         VALUES ('execute', 'allowance', ?, ?)",
    )
    .bind(allowance.id)
    .bind(format!(
        "Executed allowance {} - credited {} cents to wallet {}",
        allowance.id, allowance.amount_cents, allowance.wallet_id
    ))
    .execute(&mut *tx)
    .await
    .ok(); // Audit logs are optional, don't fail if table doesn't exist

    // 8. Check for low balance warning
    let (warning_threshold,): (i64,) =
        sqlx::query_as("SELECT warning_threshold_cents FROM wallets WHERE id = ?")
            .bind(allowance.wallet_id)
            .fetch_one(&mut *tx)
            .await?;

    if warning_threshold > 0 && new_balance <= warning_threshold {
        let warning_payload = serde_json::json!({
            "wallet_id": allowance.wallet_id,
            "balance_cents": new_balance,
            "threshold_cents": warning_threshold
        })
        .to_string();

        sqlx::query(
            "INSERT INTO notifications (event_type, payload, is_read) \
             VALUES ('low_balance', ?, 0)",
        )
        .bind(&warning_payload)
        .execute(&mut *tx)
        .await?;
    }

    // Commit the transaction
    tx.commit().await?;

    Ok(())
}
