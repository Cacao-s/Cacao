use crate::sync::discovery::PeerInfo;
use crate::sync::protocol::*;

pub struct SyncClient {
    base_url: String,
    http: reqwest::Client,
}

impl SyncClient {
    pub fn new(peer: &PeerInfo) -> Self {
        Self {
            base_url: format!("http://{}:{}", peer.ip, peer.port),
            http: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .build()
                .unwrap_or_default(),
        }
    }

    pub async fn handshake(
        &self,
        req: &SyncHandshakeRequest,
    ) -> Result<SyncHandshakeResponse, String> {
        let resp = self
            .http
            .post(format!("{}/sync/handshake", self.base_url))
            .json(req)
            .send()
            .await
            .map_err(|e| format!("Handshake request failed: {}", e))?;

        resp.json::<SyncHandshakeResponse>()
            .await
            .map_err(|e| format!("Handshake response parse failed: {}", e))
    }

    pub async fn push(&self, req: &SyncPushRequest) -> Result<SyncPushResponse, String> {
        let resp = self
            .http
            .post(format!("{}/sync/push", self.base_url))
            .json(req)
            .send()
            .await
            .map_err(|e| format!("Push request failed: {}", e))?;

        resp.json::<SyncPushResponse>()
            .await
            .map_err(|e| format!("Push response parse failed: {}", e))
    }

    pub async fn pull(&self, since: Option<&str>) -> Result<SyncPullResponse, String> {
        let mut url = format!("{}/sync/pull", self.base_url);
        if let Some(since) = since {
            url = format!("{}?since={}", url, urlencoding::encode(since));
        }

        let resp = self
            .http
            .get(&url)
            .send()
            .await
            .map_err(|e| format!("Pull request failed: {}", e))?;

        resp.json::<SyncPullResponse>()
            .await
            .map_err(|e| format!("Pull response parse failed: {}", e))
    }
}
