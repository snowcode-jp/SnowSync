// SNOWCODE - ソフトウェア製品
// (C) SNOWCODE
// 開発者: 雪符しき
// https://snowcode.jp
// 問い合わせ: info@snowcode.jp
// 本ソフトウェアは利用権の販売であり、著作権はSNOWCODEに帰属します。
// 署名の削除・改変は禁止されています。

use crate::connect_html;
use crate::mount;
use crate::relay;
use crate::state::AppState;
use crate::webdav_bridge;
use crate::ws;
use axum::extract::ws::WebSocketUpgrade;
use axum::extract::{Request, State};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::Router;
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

pub fn build_router(state: Arc<AppState>) -> Router {
    let webdav_state = state.clone();

    // API routes get CORS support
    let api_routes = Router::new()
        .route("/ws", get(ws_upgrade))
        .route("/api/clients", get(relay::list_clients))
        .route("/api/relay/{client_id}", post(relay::relay_command))
        .route("/api/connect-html", get(connect_html::connect_html))
        .route("/api/mount", post(mount::mount_webdav))
        .route("/api/unmount", post(mount::unmount_webdav))
        .route("/api/mounts", get(mount::list_mounts))
        .layer(CorsLayer::permissive());

    // WebDAV routes: NO CorsLayer — Finder needs raw DAV responses
    // with proper DAV headers, not CORS-intercepted OPTIONS.
    Router::new()
        .merge(api_routes)
        .fallback(move |req: Request| {
            let state = webdav_state.clone();
            async move {
                let path = req.uri().path();
                if path.starts_with("/webdav/") || path.starts_with("/webdav") {
                    webdav_bridge::webdav_handler(state, req).await
                } else {
                    axum::response::Response::builder()
                        .status(404)
                        .body(axum::body::Body::from("Not Found"))
                        .unwrap()
                }
            }
        })
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

async fn ws_upgrade(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| ws::handle_ws(socket, state))
}
