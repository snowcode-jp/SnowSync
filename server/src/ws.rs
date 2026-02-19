// SNOWCODE - ソフトウェア製品
// (C) SNOWCODE
// 開発者: 雪符しき
// https://snowcode.jp
// 問い合わせ: info@snowcode.jp

use axum::extract::ws::{Message, WebSocket};
use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio::sync::mpsc;

use crate::state::{AppState, ClientInfo, ConnectedClient};

pub async fn handle_ws(socket: WebSocket, state: Arc<AppState>) {
    let (mut ws_tx, mut ws_rx) = socket.split();
    let (tx, mut rx) = mpsc::unbounded_channel::<Message>();

    // Wait for the initial registration message from the Windows client
    // Expected: {"type":"register","name":"PC Name","folderName":"My Folder"}
    let registration = match ws_rx.next().await {
        Some(Ok(Message::Text(text))) => {
            match serde_json::from_str::<serde_json::Value>(&text) {
                Ok(v) => v,
                Err(_) => {
                    tracing::warn!("Invalid registration JSON");
                    return;
                }
            }
        }
        _ => {
            tracing::warn!("No registration message received");
            return;
        }
    };

    let client_id = uuid::Uuid::new_v4().to_string();
    let name = registration["name"]
        .as_str()
        .unwrap_or("Unknown PC")
        .to_string();
    let folder_name = registration["folderName"]
        .as_str()
        .unwrap_or("Unknown")
        .to_string();

    let info = ClientInfo {
        id: client_id.clone(),
        name: name.clone(),
        folder_name: folder_name.clone(),
        connected_at: chrono_now(),
    };

    // Send client_id back to Windows
    let ack = serde_json::json!({
        "type": "registered",
        "clientId": &client_id,
    });
    if tx.send(Message::Text(ack.to_string().into())).is_err() {
        return;
    }

    // Register client
    {
        let mut clients = state.clients.write().await;
        clients.insert(
            client_id.clone(),
            ConnectedClient {
                info,
                tx: tx.clone(),
            },
        );
    }
    tracing::info!("Client connected: {} ({}) - folder: {}", name, client_id, folder_name);

    // Spawn task to forward messages from channel to WebSocket
    let send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if ws_tx.send(msg).await.is_err() {
                break;
            }
        }
    });

    // Read messages from WebSocket (responses from Windows client)
    while let Some(msg) = ws_rx.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                if let Ok(value) = serde_json::from_str::<serde_json::Value>(&text) {
                    // Route response to pending request
                    if let Some(id) = value.get("id").and_then(|v| v.as_str()) {
                        let mut pending = state.pending.write().await;
                        if let Some(responder) = pending.remove(id) {
                            let _ = responder.send(value);
                        }
                    }
                }
            }
            Ok(Message::Close(_)) => break,
            Err(e) => {
                tracing::warn!("WS error from {}: {}", client_id, e);
                break;
            }
            _ => {}
        }
    }

    // Cleanup
    send_task.abort();
    {
        let mut clients = state.clients.write().await;
        clients.remove(&client_id);
    }
    tracing::info!("Client disconnected: {} ({})", name, client_id);
}

fn chrono_now() -> String {
    // Simple ISO 8601 timestamp without chrono dependency
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    format!("{}", now)
}
