use crate::sync::apply;
use crate::sync::client::SyncClient;
use crate::sync::discovery::{MdnsBroadcaster, MdnsDiscovery};
use crate::sync::merge;
use crate::sync::protocol::*;
use crate::sync::server::SyncServer;
use sqlx::SqlitePool;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct SyncCoordinator {
    pool: SqlitePool,
    role: String,
    family_uuid: String,
    device_uuid: String,
    server: Option<SyncServer>,
    broadcaster: Option<MdnsBroadcaster>,
    status: Arc<Mutex<SyncStatus>>,
}

impl SyncCoordinator {
    /// Giver: start HTTP server + mDNS broadcast
    pub async fn start_as_giver(
        pool: SqlitePool,
        family_uuid: &str,
        device_name: &str,
    ) -> Result<Self, String> {
        let status = Arc::new(Mutex::new(SyncStatus::default()));

        // Start HTTP sync server
        let server = SyncServer::start(pool.clone(), family_uuid.to_string()).await?;
        let port = server.port();

        // Start mDNS broadcast
        let broadcaster = MdnsBroadcaster::start(family_uuid, device_name, port)?;

        {
            let mut s = status.lock().await;
            s.state = "connected".to_string();
        }

        log::info!("Sync coordinator started as Giver on port {}", port);

        Ok(Self {
            pool,
            role: "giver".to_string(),
            family_uuid: family_uuid.to_string(),
            device_uuid: String::new(),
            server: Some(server),
            broadcaster: Some(broadcaster),
            status,
        })
    }

    /// Baby: discover peer, handshake, then sync
    pub async fn start_as_baby(
        pool: SqlitePool,
        family_uuid: &str,
        device_uuid: &str,
    ) -> Result<Self, String> {
        let status = Arc::new(Mutex::new(SyncStatus {
            state: "discovering".to_string(),
            ..Default::default()
        }));

        Ok(Self {
            pool,
            role: "baby".to_string(),
            family_uuid: family_uuid.to_string(),
            device_uuid: device_uuid.to_string(),
            server: None,
            broadcaster: None,
            status,
        })
    }

    /// Baby: perform a full sync cycle (discover → handshake → push → pull)
    pub async fn sync_now(&self) -> Result<(), String> {
        if self.role != "baby" {
            return Ok(()); // Giver doesn't initiate sync
        }

        // Update status: discovering
        {
            let mut s = self.status.lock().await;
            s.state = "discovering".to_string();
            s.error_message = None;
        }

        // Discover Giver
        let peers = MdnsDiscovery::search(&self.family_uuid, 10)?;
        if peers.is_empty() {
            let mut s = self.status.lock().await;
            s.state = "disconnected".to_string();
            s.error_message = Some("No Giver device found on this network".to_string());
            return Err("PEER_NOT_FOUND".to_string());
        }

        let peer = &peers[0];
        let client = SyncClient::new(peer);

        // Handshake
        let handshake_resp = client
            .handshake(&SyncHandshakeRequest {
                device_uuid: self.device_uuid.clone(),
                family_uuid: self.family_uuid.clone(),
                pairing_code: None,
            })
            .await?;

        if !handshake_resp.success {
            let mut s = self.status.lock().await;
            s.state = "error".to_string();
            s.error_message = Some("Handshake failed".to_string());
            return Err("HANDSHAKE_FAILED".to_string());
        }

        {
            let mut s = self.status.lock().await;
            s.state = "syncing".to_string();
            s.peer_device_name = Some(peer.device_name.clone());
        }

        // Push local changes to Giver
        let last_sync = self.get_last_sync_timestamp().await;
        let local_changes = merge::collect_changes_since(&self.pool, &last_sync).await?;

        if !local_changes.is_empty() {
            let push_resp = client
                .push(&SyncPushRequest {
                    device_uuid: self.device_uuid.clone(),
                    changes: local_changes,
                })
                .await?;

            log::info!(
                "Push result: {} accepted, {} rejected",
                push_resp.accepted.len(),
                push_resp.rejected.len()
            );
        }

        // Pull changes from Giver
        let pull_resp = client.pull(Some(&last_sync)).await?;
        if !pull_resp.changes.is_empty() {
            let affected = apply::apply_changes(&self.pool, pull_resp.changes).await?;
            log::info!("Applied changes, {} wallets affected", affected.len());
        }

        // Update status
        let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        {
            let mut s = self.status.lock().await;
            s.state = "connected".to_string();
            s.last_sync_at = Some(now);
            s.error_message = None;
        }

        Ok(())
    }

    pub async fn stop(&mut self) {
        if let Some(broadcaster) = self.broadcaster.take() {
            broadcaster.stop();
        }
        if let Some(mut server) = self.server.take() {
            server.stop();
        }

        let mut s = self.status.lock().await;
        s.state = "disconnected".to_string();
    }

    pub async fn status(&self) -> SyncStatus {
        self.status.lock().await.clone()
    }

    async fn get_last_sync_timestamp(&self) -> String {
        sqlx::query_scalar::<_, String>(
            "SELECT value FROM sync_state WHERE key = 'last_sync_timestamp'",
        )
        .fetch_optional(&self.pool)
        .await
        .ok()
        .flatten()
        .unwrap_or_else(|| "1970-01-01 00:00:00".to_string())
    }
}
