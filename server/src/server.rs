// SNOWCODE - ソフトウェア製品
// (C) SNOWCODE
// 開発者: 雪符しき
// https://snowcode.jp
// 問い合わせ: info@snowcode.jp

use crate::connect_html;
use crate::mount;
use crate::relay;
use crate::state::AppState;
use crate::webdav_bridge;
use crate::ws;
use axum::extract::ws::WebSocketUpgrade;
use axum::extract::{Request, State};
use axum::http::{header, HeaderValue, Method, StatusCode};
use axum::middleware::{self, Next};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::Router;
use std::sync::Arc;
use tower_http::classify::{ServerErrorsAsFailures, SharedClassifier};
use tower_http::cors::CorsLayer;
use tower_http::trace::{self, TraceLayer};

/// Authentication middleware: validates Bearer token on protected routes.
async fn auth_middleware(
    State(state): State<Arc<AppState>>,
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let token = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .unwrap_or("");

    if token != state.api_token {
        tracing::warn!(
            "Auth rejected: {} {} (invalid token)",
            req.method(),
            req.uri().path()
        );
        return Err(StatusCode::UNAUTHORIZED);
    }

    Ok(next.run(req).await)
}

pub fn build_router(state: Arc<AppState>) -> Router {
    let webdav_state = state.clone();

    // CORS: allow localhost + LAN origins (not fully permissive)
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION])
        .allow_origin([
            "http://localhost:17100".parse::<HeaderValue>().unwrap(),
            "http://127.0.0.1:17100".parse::<HeaderValue>().unwrap(),
        ]);

    // Public routes (no auth required)
    let public_routes = Router::new()
        .route("/", get(connect_html::download_page))
        .route("/ws", get(ws_upgrade))
        .route("/api/connect-html", get(connect_html::connect_html));

    // Protected routes (require Bearer token)
    let protected_routes = Router::new()
        .route("/api/clients", get(relay::list_clients))
        .route("/api/relay/{client_id}", post(relay::relay_command))
        .route("/api/mount", post(mount::mount_webdav))
        .route("/api/unmount", post(mount::unmount_webdav))
        .route("/api/mounts", get(mount::list_mounts))
        .layer(middleware::from_fn_with_state(state.clone(), auth_middleware));

    let api_routes = Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .layer(cors);

    // Custom TraceLayer: downgrade WebDAV failures from ERROR to DEBUG.
    // Finder probes for many non-existent files (.DS_Store, .Spotlight-V100, etc.)
    // on mount, which produces 404/500 via the relay — these are expected and
    // should not clutter the log as ERROR.
    let trace_layer = TraceLayer::new(SharedClassifier::new(ServerErrorsAsFailures::default()))
        .on_failure(trace::DefaultOnFailure::new().level(tracing::Level::DEBUG));

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
        .layer(trace_layer)
        .with_state(state)
}

async fn ws_upgrade(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| ws::handle_ws(socket, state))
}
