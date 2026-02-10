use crate::sync::merge;
use crate::sync::protocol::*;
use axum::{
    Json, Router,
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use sqlx::SqlitePool;
use tokio::net::TcpListener;

#[derive(Clone)]
struct AppState {
    pool: SqlitePool,
    family_uuid: String,
}

pub struct SyncServer {
    shutdown_tx: Option<tokio::sync::oneshot::Sender<()>>,
    port: u16,
}

impl SyncServer {
    pub async fn start(pool: SqlitePool, family_uuid: String) -> Result<Self, String> {
        let port = find_available_port().await.map_err(|e| e.to_string())?;

        let state = AppState { pool, family_uuid };

        let app = Router::new()
            .route("/sync/handshake", post(handshake_handler))
            .route("/sync/push", post(push_handler))
            .route("/sync/pull", get(pull_handler))
            .with_state(state);

        let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel::<()>();
        let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
            .await
            .map_err(|e| e.to_string())?;

        log::info!("Sync server started on port {}", port);

        tokio::spawn(async move {
            axum::serve(listener, app)
                .with_graceful_shutdown(async {
                    shutdown_rx.await.ok();
                })
                .await
                .ok();
        });

        Ok(Self {
            shutdown_tx: Some(shutdown_tx),
            port,
        })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn stop(&mut self) {
        if let Some(tx) = self.shutdown_tx.take() {
            tx.send(()).ok();
            log::info!("Sync server stopped");
        }
    }
}

async fn find_available_port() -> Result<u16, std::io::Error> {
    let listener = TcpListener::bind("0.0.0.0:0").await?;
    let port = listener.local_addr()?.port();
    drop(listener);
    Ok(port)
}

async fn handshake_handler(
    State(state): State<AppState>,
    Json(req): Json<SyncHandshakeRequest>,
) -> impl IntoResponse {
    // Verify family_uuid matches
    if req.family_uuid != state.family_uuid {
        return Json(SyncHandshakeResponse {
            success: false,
            family_name: None,
        });
    }

    // If pairing code is provided, validate it
    if let Some(code) = &req.pairing_code {
        let valid =
            crate::services::pairing_service::validate_pairing_code(&state.pool, code).await;
        if valid.is_err() {
            return Json(SyncHandshakeResponse {
                success: false,
                family_name: None,
            });
        }
    }

    // Get family name
    let family_name = sqlx::query_scalar::<_, String>(
        "SELECT name FROM families WHERE uuid = ? AND is_deleted = 0",
    )
    .bind(&req.family_uuid)
    .fetch_optional(&state.pool)
    .await
    .ok()
    .flatten();

    // Record paired device
    let _ = sqlx::query(
        "INSERT OR REPLACE INTO paired_devices (device_uuid, display_name, role, family_id, last_seen_at, is_active) \
         SELECT ?, 'Baby Device', 'baby', id, datetime('now'), 1 FROM families WHERE uuid = ?"
    )
    .bind(&req.device_uuid)
    .bind(&req.family_uuid)
    .execute(&state.pool)
    .await;

    Json(SyncHandshakeResponse {
        success: true,
        family_name,
    })
}

async fn push_handler(
    State(state): State<AppState>,
    Json(req): Json<SyncPushRequest>,
) -> impl IntoResponse {
    // Update last_seen for the device
    let _ = sqlx::query(
        "UPDATE paired_devices SET last_seen_at = datetime('now') WHERE device_uuid = ?",
    )
    .bind(&req.device_uuid)
    .execute(&state.pool)
    .await;

    match merge::merge_changes(&state.pool, req.changes).await {
        Ok(result) => (
            StatusCode::OK,
            Json(SyncPushResponse {
                accepted: result.accepted,
                rejected: result.rejected,
            }),
        ),
        Err(e) => {
            log::error!("Merge error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(SyncPushResponse {
                    accepted: vec![],
                    rejected: vec![],
                }),
            )
        }
    }
}

#[derive(serde::Deserialize)]
struct PullParams {
    since: Option<String>,
}

async fn pull_handler(
    State(state): State<AppState>,
    Query(params): Query<PullParams>,
) -> impl IntoResponse {
    let since = params
        .since
        .unwrap_or_else(|| "1970-01-01 00:00:00".to_string());

    match merge::collect_changes_since(&state.pool, &since).await {
        Ok(changes) => (
            StatusCode::OK,
            Json(SyncPullResponse {
                changes,
                server_timestamp: chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            }),
        ),
        Err(e) => {
            log::error!("Pull error: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(SyncPullResponse {
                    changes: vec![],
                    server_timestamp: chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                }),
            )
        }
    }
}
