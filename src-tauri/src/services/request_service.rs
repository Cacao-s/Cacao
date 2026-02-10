use crate::error::AppError;
use crate::models::params::{CreateRequestParams, UpdateRequestParams};
use crate::models::request::Request;
use sqlx::SqlitePool;

pub async fn create_request(
    pool: &SqlitePool,
    params: &CreateRequestParams,
) -> Result<Request, AppError> {
    if params.amount_cents <= 0 {
        return Err(AppError::validation("Amount must be positive"));
    }

    // Verify wallet exists and belongs to the family
    let wallet_exists: bool = sqlx::query_scalar(
        "SELECT COUNT(*) > 0 FROM wallets WHERE id = ? AND family_id = ? AND is_deleted = 0 AND status = 'active'"
    )
    .bind(params.wallet_id)
    .bind(params.family_id)
    .fetch_one(pool).await?;

    if !wallet_exists {
        return Err(AppError::not_found("wallet"));
    }

    let request = sqlx::query_as::<_, Request>(
        "INSERT INTO requests (family_id, requester_member_id, wallet_id, amount_cents, category, notes, status) \
         VALUES (?, ?, ?, ?, ?, ?, 'draft') RETURNING *"
    )
    .bind(params.family_id)
    .bind(params.requester_member_id)
    .bind(params.wallet_id)
    .bind(params.amount_cents)
    .bind(&params.category)
    .bind(&params.notes)
    .fetch_one(pool).await?;

    Ok(request)
}

pub async fn update_request(
    pool: &SqlitePool,
    id: i64,
    params: &UpdateRequestParams,
) -> Result<Request, AppError> {
    let request = get_request(pool, id).await?;
    if request.status != "draft" {
        return Err(AppError::new(
            "REQUEST_INVALID_STATUS",
            "Only draft requests can be edited",
        ));
    }

    if let Some(amount) = params.amount_cents
        && amount <= 0
    {
        return Err(AppError::validation("Amount must be positive"));
    }

    let amount = params.amount_cents.unwrap_or(request.amount_cents);
    let category = params.category.as_deref().or(request.category.as_deref());
    let notes = params.notes.as_deref().or(request.notes.as_deref());

    sqlx::query(
        "UPDATE requests SET amount_cents = ?, category = ?, notes = ?, updated_at = datetime('now') WHERE id = ?"
    )
    .bind(amount)
    .bind(category)
    .bind(notes)
    .bind(id)
    .execute(pool).await?;

    get_request(pool, id).await
}

pub async fn submit_request(pool: &SqlitePool, id: i64) -> Result<Request, AppError> {
    let request = get_request(pool, id).await?;
    if request.status != "draft" {
        return Err(AppError::new(
            "REQUEST_INVALID_STATUS",
            "Only draft requests can be submitted",
        ));
    }

    sqlx::query(
        "UPDATE requests SET status = 'pending', sync_version = sync_version + 1, updated_at = datetime('now') WHERE id = ?"
    )
    .bind(id)
    .execute(pool).await?;

    get_request(pool, id).await
}

pub async fn approve_request(
    pool: &SqlitePool,
    id: i64,
    approver_member_id: i64,
) -> Result<Request, AppError> {
    let request = get_request(pool, id).await?;
    if request.status != "pending" {
        if request.status == "approved" || request.status == "rejected" {
            return Err(AppError::new(
                "REQUEST_ALREADY_DECIDED",
                "This request has already been decided",
            ));
        }
        return Err(AppError::new(
            "REQUEST_INVALID_STATUS",
            "Only pending requests can be approved",
        ));
    }

    let mut tx = pool.begin().await?;

    // Check wallet balance
    let balance: i64 =
        sqlx::query_scalar("SELECT balance_cents FROM wallets WHERE id = ? AND is_deleted = 0")
            .bind(request.wallet_id)
            .fetch_one(&mut *tx)
            .await?;

    if balance < request.amount_cents {
        return Err(AppError::new(
            "INSUFFICIENT_BALANCE",
            "Wallet has insufficient balance",
        ));
    }

    // Debit wallet
    sqlx::query(
        "UPDATE wallets SET balance_cents = balance_cents - ?, sync_version = sync_version + 1, updated_at = datetime('now') WHERE id = ?"
    )
    .bind(request.amount_cents)
    .bind(request.wallet_id)
    .execute(&mut *tx).await?;

    // Create debit transaction
    sqlx::query(
        "INSERT INTO transactions (family_id, wallet_id, type, amount_cents, source_type, source_id, category, occurred_at) \
         VALUES (?, ?, 'debit', ?, 'request', ?, ?, datetime('now'))"
    )
    .bind(request.family_id)
    .bind(request.wallet_id)
    .bind(request.amount_cents)
    .bind(request.id)
    .bind(&request.category)
    .execute(&mut *tx).await?;

    // Update request status
    sqlx::query(
        "UPDATE requests SET status = 'approved', decision_by_member_id = ?, decision_at = datetime('now'), \
         sync_version = sync_version + 1, updated_at = datetime('now') WHERE id = ?"
    )
    .bind(approver_member_id)
    .bind(id)
    .execute(&mut *tx).await?;

    // Create notification
    let payload = serde_json::json!({
        "request_id": request.id,
        "amount_cents": request.amount_cents,
        "wallet_id": request.wallet_id,
    })
    .to_string();

    sqlx::query("INSERT INTO notifications (event_type, payload) VALUES ('request_approved', ?)")
        .bind(&payload)
        .execute(&mut *tx)
        .await?;

    // Audit log
    let metadata = serde_json::json!({
        "request_id": request.id,
        "amount_cents": request.amount_cents,
        "approver_member_id": approver_member_id,
    })
    .to_string();

    sqlx::query(
        "INSERT INTO audit_logs (family_id, action, resource_type, resource_id, metadata) \
         VALUES (?, 'approve_request', 'request', ?, ?)",
    )
    .bind(request.family_id)
    .bind(request.id.to_string())
    .bind(&metadata)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    get_request(pool, id).await
}

pub async fn reject_request(
    pool: &SqlitePool,
    id: i64,
    reason: &str,
    rejector_member_id: i64,
) -> Result<Request, AppError> {
    let reason = reason.trim();
    if reason.is_empty() {
        return Err(AppError::new(
            "REJECTION_REASON_REQUIRED",
            "A reason is required when rejecting a request",
        ));
    }

    let request = get_request(pool, id).await?;
    if request.status != "pending" {
        if request.status == "approved" || request.status == "rejected" {
            return Err(AppError::new(
                "REQUEST_ALREADY_DECIDED",
                "This request has already been decided",
            ));
        }
        return Err(AppError::new(
            "REQUEST_INVALID_STATUS",
            "Only pending requests can be rejected",
        ));
    }

    sqlx::query(
        "UPDATE requests SET status = 'rejected', rejection_reason = ?, decision_by_member_id = ?, \
         decision_at = datetime('now'), sync_version = sync_version + 1, updated_at = datetime('now') WHERE id = ?"
    )
    .bind(reason)
    .bind(rejector_member_id)
    .bind(id)
    .execute(pool).await?;

    // Create notification
    let payload = serde_json::json!({
        "request_id": request.id,
        "amount_cents": request.amount_cents,
        "reason": reason,
    })
    .to_string();

    sqlx::query("INSERT INTO notifications (event_type, payload) VALUES ('request_rejected', ?)")
        .bind(&payload)
        .execute(pool)
        .await?;

    get_request(pool, id).await
}

pub async fn cancel_request(pool: &SqlitePool, id: i64) -> Result<Request, AppError> {
    let request = get_request(pool, id).await?;
    if request.status != "draft" && request.status != "pending" {
        return Err(AppError::new(
            "REQUEST_INVALID_STATUS",
            "Only draft or pending requests can be cancelled",
        ));
    }

    sqlx::query(
        "UPDATE requests SET status = 'cancelled', sync_version = sync_version + 1, updated_at = datetime('now') WHERE id = ?"
    )
    .bind(id)
    .execute(pool).await?;

    get_request(pool, id).await
}

pub async fn list_requests(
    pool: &SqlitePool,
    family_id: i64,
    status: Option<&str>,
) -> Result<Vec<Request>, AppError> {
    let requests = if let Some(status) = status {
        sqlx::query_as::<_, Request>(
            "SELECT * FROM requests WHERE family_id = ? AND status = ? AND is_deleted = 0 ORDER BY created_at DESC"
        )
        .bind(family_id)
        .bind(status)
        .fetch_all(pool).await?
    } else {
        sqlx::query_as::<_, Request>(
            "SELECT * FROM requests WHERE family_id = ? AND is_deleted = 0 ORDER BY created_at DESC"
        )
        .bind(family_id)
        .fetch_all(pool).await?
    };
    Ok(requests)
}

pub async fn get_request(pool: &SqlitePool, id: i64) -> Result<Request, AppError> {
    sqlx::query_as::<_, Request>("SELECT * FROM requests WHERE id = ? AND is_deleted = 0")
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::not_found("request"))
}
