// SNOWCODE - ソフトウェア製品
// (C) SNOWCODE
// 開発者: 雪符しき
// https://snowcode.jp
// 問い合わせ: info@snowcode.jp

mod config;
mod connect_html;
mod mount;
mod relay;
mod server;
mod state;
mod tls;
mod webdav_bridge;
mod ws;

use anyhow::Result;
use config::AppConfig;
use state::AppState;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "ljc_server=info,tower_http=info".into()),
        )
        .init();

    let config = AppConfig::from_env();
    let port = config.port;
    let https_port = port + 1; // 17201
    let bind = config.bind_address.clone();

    let state = AppState::new(port, config.allowed_mount_base.clone());
    let app = server::build_router(state.clone());

    let local_ip = local_ip_address::local_ip()
        .map(|ip| ip.to_string())
        .unwrap_or_else(|_| "localhost".to_string());

    // --- HTTP listener (API, WebSocket, WebDAV fallback) ---
    let http_addr = format!("{}:{}", bind, port);
    let http_listener = TcpListener::bind(&http_addr).await?;

    tracing::info!("===========================================");
    tracing::info!("  SnowSync Relay Server");
    tracing::info!("===========================================");
    tracing::info!("  WebSocket: ws://{}:{}/ws", local_ip, port);
    tracing::info!("  API:       http://{}:{}/api/", local_ip, port);
    tracing::info!("  WebDAV:    https://{}:{}/webdav/<client_id>/", local_ip, https_port);
    tracing::info!("===========================================");
    tracing::info!("");
    tracing::info!("  API Token: {}", state.api_token);
    tracing::info!("  Mount base: {}", config.allowed_mount_base);
    tracing::info!("");
    tracing::info!("  Finderマウント: Cmd+K -> https://{}:{}/webdav/<client_id>/", local_ip, https_port);
    tracing::info!("  接続HTML: http://{}:{}/api/connect-html", local_ip, port);

    // --- HTTPS listener (WebDAV for Finder on macOS Tahoe) ---
    let tls_config = tls::make_tls_config(&local_ip)?;

    let https_addr = format!("{}:{}", bind, https_port);
    let https_app = server::build_router(state);
    tracing::info!("  HTTPS WebDAV listening on port {}", https_port);

    let tls_acceptor = tokio_rustls::TlsAcceptor::from(tls_config);
    let https_listener = TcpListener::bind(&https_addr).await?;

    // Spawn HTTPS server
    let https_handle = tokio::spawn(async move {
        loop {
            let (stream, addr) = match https_listener.accept().await {
                Ok(s) => s,
                Err(e) => {
                    tracing::warn!("HTTPS accept error: {}", e);
                    continue;
                }
            };
            let acceptor = tls_acceptor.clone();
            let app = https_app.clone();
            tokio::spawn(async move {
                match acceptor.accept(stream).await {
                    Ok(tls_stream) => {
                        let io = hyper_util::rt::TokioIo::new(tls_stream);
                        let service = hyper_util::service::TowerToHyperService::new(app);
                        if let Err(e) = hyper_util::server::conn::auto::Builder::new(
                            hyper_util::rt::TokioExecutor::new(),
                        )
                        .serve_connection(io, service)
                        .await
                        {
                            tracing::debug!("HTTPS connection error from {}: {}", addr, e);
                        }
                    }
                    Err(e) => {
                        tracing::debug!("TLS handshake error from {}: {}", addr, e);
                    }
                }
            });
        }
    });

    // HTTP server (main thread)
    tokio::select! {
        r = axum::serve(http_listener, app) => {
            if let Err(e) = r {
                tracing::error!("HTTP server error: {}", e);
            }
        }
        r = https_handle => {
            if let Err(e) = r {
                tracing::error!("HTTPS server error: {}", e);
            }
        }
    }

    Ok(())
}
