// SNOWCODE - ソフトウェア製品
// (C) SNOWCODE
// 開発者: 雪符しき
// https://snowcode.jp
// 問い合わせ: info@snowcode.jp

use axum::extract::ws::Message;
use serde::Serialize;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, oneshot, RwLock};

#[derive(Debug, Clone, Serialize)]
pub struct ClientInfo {
    pub id: String,
    pub name: String,
    pub folder_name: String,
    pub connected_at: String,
}

pub struct ConnectedClient {
    pub info: ClientInfo,
    pub tx: mpsc::UnboundedSender<Message>,
}

pub type PendingResponder = oneshot::Sender<serde_json::Value>;

pub struct AppState {
    pub clients: RwLock<HashMap<String, ConnectedClient>>,
    pub pending: RwLock<HashMap<String, PendingResponder>>,
    /// HTTP port (e.g. 17200)
    pub port: u16,
    /// Bearer token for API authentication (auto-generated on startup)
    pub api_token: String,
    /// Allowed base directory for WebDAV mounts (e.g. ~/Public/mount)
    pub allowed_mount_base: String,
}

impl AppState {
    pub fn new(port: u16, allowed_mount_base: String) -> Arc<Self> {
        let token = uuid::Uuid::new_v4().to_string();
        Arc::new(Self {
            clients: RwLock::new(HashMap::new()),
            pending: RwLock::new(HashMap::new()),
            port,
            api_token: token,
            allowed_mount_base,
        })
    }
}
