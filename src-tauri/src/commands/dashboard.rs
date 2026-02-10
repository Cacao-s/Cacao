use crate::error::AppError;
use crate::models::family::Family;
use crate::models::profile::Profile;
use crate::models::transaction::Transaction;
use crate::models::wallet::Wallet;
use serde::Serialize;
use sqlx::SqlitePool;
use tauri::State;

#[derive(Debug, Serialize, Clone)]
pub struct RequestSummary {
    pub pending: i64,
    pub approved: i64,
    pub rejected: i64,
}

#[derive(Debug, Serialize, Clone)]
pub struct DashboardData {
    pub profile: Profile,
    pub family: Option<Family>,
    pub wallets: Vec<Wallet>,
    pub recent_transactions: Vec<Transaction>,
    pub request_summary: RequestSummary,
    pub pending_requests_count: i64,
    pub unread_notification_count: i64,
}

#[tauri::command]
pub async fn get_dashboard_data(pool: State<'_, SqlitePool>) -> Result<DashboardData, AppError> {
    let profile =
        sqlx::query_as::<_, Profile>("SELECT * FROM profiles WHERE is_deleted = 0 LIMIT 1")
            .fetch_optional(&*pool)
            .await?
            .ok_or_else(|| AppError::new("NOT_SETUP", "Device is not set up"))?;

    let family = sqlx::query_as::<_, Family>("SELECT * FROM families WHERE is_deleted = 0 LIMIT 1")
        .fetch_optional(&*pool)
        .await?;

    let family_id = family.as_ref().map(|f| f.id).unwrap_or(0);

    let wallets = sqlx::query_as::<_, Wallet>(
        "SELECT * FROM wallets WHERE family_id = ? AND is_deleted = 0 AND status = 'active' ORDER BY created_at ASC"
    )
    .bind(family_id)
    .fetch_all(&*pool).await?;

    let recent_transactions = sqlx::query_as::<_, Transaction>(
        "SELECT * FROM transactions WHERE family_id = ? AND is_deleted = 0 ORDER BY occurred_at DESC LIMIT 5"
    )
    .bind(family_id)
    .fetch_all(&*pool).await?;

    let (pending,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM requests WHERE family_id = ? AND status = 'pending' AND is_deleted = 0"
    )
    .bind(family_id)
    .fetch_one(&*pool).await?;

    let (approved,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM requests WHERE family_id = ? AND status = 'approved' AND is_deleted = 0"
    )
    .bind(family_id)
    .fetch_one(&*pool).await?;

    let (rejected,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM requests WHERE family_id = ? AND status = 'rejected' AND is_deleted = 0"
    )
    .bind(family_id)
    .fetch_one(&*pool).await?;

    let (unread_count,): (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM notifications WHERE is_read = 0")
            .fetch_one(&*pool)
            .await?;

    Ok(DashboardData {
        profile,
        family,
        wallets,
        recent_transactions,
        request_summary: RequestSummary {
            pending,
            approved,
            rejected,
        },
        pending_requests_count: pending,
        unread_notification_count: unread_count,
    })
}
