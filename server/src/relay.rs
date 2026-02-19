// SNOWCODE - ソフトウェア製品
// (C) SNOWCODE
// 開発者: 雪符しき
// https://snowcode.jp
// 問い合わせ: info@snowcode.jp

use axum::extract::ws::Message;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::Json;
use serde_json::{json, Value};
use std::sync::Arc;
use tokio::sync::oneshot;
use tokio::time::{timeout, Duration};

use crate::state::AppState;

/// POST /api/relay/{client_id}
/// Body: {"type": "readdir", "path": "/", ...}
/// Sends the command to the Windows client via WS, waits for response.
pub async fn relay_command(
    State(state): State<Arc<AppState>>,
    Path(client_id): Path<String>,
    Json(mut body): Json<Value>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    // Generate request ID
    let request_id = uuid::Uuid::new_v4().to_string();
    body["id"] = json!(request_id);

    // Get the client's WS sender
    let tx = {
        let clients = state.clients.read().await;
        match clients.get(&client_id) {
            Some(client) => client.tx.clone(),
            None => {
                return Err((
                    StatusCode::NOT_FOUND,
                    Json(json!({"error": "Client not found"})),
                ));
            }
        }
    };

    // Register pending request
    let (resp_tx, resp_rx) = oneshot::channel();
    {
        let mut pending = state.pending.write().await;
        pending.insert(request_id.clone(), resp_tx);
    }

    // Send command to Windows client
    if tx
        .send(Message::Text(body.to_string().into()))
        .is_err()
    {
        let mut pending = state.pending.write().await;
        pending.remove(&request_id);
        return Err((
            StatusCode::BAD_GATEWAY,
            Json(json!({"error": "Failed to send to client"})),
        ));
    }

    // Wait for response with timeout (30s for large file reads)
    match timeout(Duration::from_secs(30), resp_rx).await {
        Ok(Ok(response)) => Ok(Json(response)),
        Ok(Err(_)) => Err((
            StatusCode::BAD_GATEWAY,
            Json(json!({"error": "Client connection lost"})),
        )),
        Err(_) => {
            let mut pending = state.pending.write().await;
            pending.remove(&request_id);
            Err((
                StatusCode::GATEWAY_TIMEOUT,
                Json(json!({"error": "Request timed out"})),
            ))
        }
    }
}

/// GET /api/clients
/// Returns list of connected Windows clients.
pub async fn list_clients(
    State(state): State<Arc<AppState>>,
) -> Json<Value> {
    let clients = state.clients.read().await;
    let list: Vec<_> = clients.values().map(|c| &c.info).collect();
    Json(json!(list))
}
