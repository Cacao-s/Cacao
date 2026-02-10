use crate::models::allowance::Allowance;
use crate::services::allowance_service;
use sqlx::SqlitePool;
use std::time::Duration;
use tokio_util::sync::CancellationToken;

/// Runs only on Giver device.
/// Checks for due allowances every 60 seconds while app is in foreground.
pub async fn start(pool: SqlitePool, cancel: CancellationToken) {
    log::info!("Allowance runner started");

    loop {
        tokio::select! {
            _ = cancel.cancelled() => {
                log::info!("Allowance runner stopped");
                break;
            }
            _ = tokio::time::sleep(Duration::from_secs(60)) => {
                let due = sqlx::query_as::<_, Allowance>(
                    "SELECT * FROM allowances \
                     WHERE status = 'active' AND next_run_at <= datetime('now') AND is_deleted = 0"
                )
                .fetch_all(&pool)
                .await;

                match due {
                    Ok(allowances) => {
                        for a in allowances {
                            if let Err(e) = allowance_service::execute_allowance(&pool, &a).await {
                                log::error!("Allowance {} execute failed: {:?}", a.id, e);
                            } else {
                                log::info!("Allowance {} executed successfully", a.id);
                            }
                        }
                    }
                    Err(e) => {
                        log::error!("Failed to query due allowances: {}", e);
                    }
                }
            }
        }
    }
}
