// SNOWCODE - ソフトウェア製品
// (C) SNOWCODE
// 開発者: 雪符しき
// https://snowcode.jp
// 問い合わせ: info@snowcode.jp

use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Json;
use serde::Deserialize;
use serde_json::{json, Value};
use std::sync::Arc;

use crate::state::AppState;

#[derive(Deserialize)]
pub struct MountRequest {
    pub client_id: String,
    pub mount_path: String,
}

#[derive(Deserialize)]
pub struct UnmountRequest {
    pub mount_path: String,
}

/// Expand ~ to home directory
fn expand_tilde(path: &str) -> String {
    if path.starts_with("~/") || path == "~" {
        if let Ok(home) = std::env::var("HOME") {
            return path.replacen('~', &home, 1);
        }
    }
    path.to_string()
}

/// Check if path is safe (no traversal, within allowed base)
fn validate_mount_path(path: &str, allowed_base: &str) -> Result<String, String> {
    // Reject paths containing ".."
    if path.contains("..") {
        return Err("Path must not contain '..'".to_string());
    }

    let expanded = expand_tilde(path);
    let allowed_expanded = expand_tilde(allowed_base);

    // Normalize: ensure both end consistently
    let expanded_clean = expanded.trim_end_matches('/');
    let allowed_clean = allowed_expanded.trim_end_matches('/');

    // Must be under allowed base
    if !expanded_clean.starts_with(allowed_clean) {
        return Err(format!(
            "Mount path must be under '{}' (got '{}')",
            allowed_base, path
        ));
    }

    Ok(expanded)
}

/// Validate that a string looks like a UUID (8-4-4-4-12 hex)
fn is_valid_uuid(s: &str) -> bool {
    uuid::Uuid::parse_str(s).is_ok()
}

/// POST /api/mount
/// Body: {"client_id": "xxx", "mount_path": "~/Public/mount"}
/// Mounts the WebDAV share. Uses 127.0.0.1 HTTP (port 17200) which bypasses
/// macOS Tahoe's HTTP WebDAV block (only blocks non-loopback HTTP).
pub async fn mount_webdav(
    State(state): State<Arc<AppState>>,
    Json(body): Json<MountRequest>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let client_id = &body.client_id;

    // Validate client_id is UUID format
    if !is_valid_uuid(client_id) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Invalid client_id format"})),
        ));
    }

    // Validate mount path is under allowed base directory
    let base_mount_path = match validate_mount_path(&body.mount_path, &state.allowed_mount_base) {
        Ok(path) => path,
        Err(e) => {
            return Err((
                StatusCode::FORBIDDEN,
                Json(json!({"error": e})),
            ));
        }
    };
    let base_mount_path = &base_mount_path;

    // Check client exists
    {
        let clients = state.clients.read().await;
        if !clients.contains_key(client_id) {
            return Err((
                StatusCode::NOT_FOUND,
                Json(json!({"error": "クライアントが見つかりません"})),
            ));
        }
    }

    // Create mount point: base_mount_path/ljc-{client_id_short}
    let short_id = &client_id[..8.min(client_id.len())];
    let mount_point = format!("{}/ljc-{}", base_mount_path.trim_end_matches('/'), short_id);

    // Create directory
    let mkdir_result = tokio::process::Command::new("mkdir")
        .args(["-p", &mount_point])
        .output()
        .await;

    match mkdir_result {
        Ok(output) if !output.status.success() => {
            let stderr = String::from_utf8_lossy(&output.stderr);
            tracing::error!("mkdir failed: {}", stderr);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": format!("ディレクトリ作成失敗: {}", stderr)})),
            ));
        }
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": format!("mkdir実行エラー: {}", e)})),
            ));
        }
        _ => {}
    }

    // Use 127.0.0.1 HTTP — macOS Tahoe blocks HTTP WebDAV for non-loopback
    // addresses, but allows loopback (127.0.0.1). Since the server runs on
    // the same Mac, we can always connect via loopback.
    let port = state.port;
    let https_port = port + 1;
    let webdav_url = format!("http://127.0.0.1:{}/webdav/{}/", port, client_id);

    tracing::info!("Mounting WebDAV: {} -> {}", webdav_url, mount_point);

    // Strategy 1: mount_webdav -S via 127.0.0.1 HTTP
    tracing::info!("Trying mount_webdav -S (127.0.0.1 HTTP) ...");
    let mount_result = tokio::process::Command::new("mount_webdav")
        .args(["-S", &webdav_url, &mount_point])
        .output()
        .await;

    match &mount_result {
        Ok(output) if output.status.success() => {
            tracing::info!("mount_webdav success: {}", mount_point);
            return Ok(Json(json!({
                "ok": true,
                "mount_point": mount_point,
                "webdav_url": webdav_url,
            })));
        }
        Ok(output) => {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let code = output.status.code().unwrap_or(-1);
            tracing::warn!("mount_webdav 127.0.0.1 failed (exit {}): {}", code, stderr);
        }
        Err(e) => {
            tracing::warn!("mount_webdav exec error: {}", e);
        }
    }

    // Strategy 2: mount_webdav via HTTPS (port + 1)
    let https_url = format!("https://127.0.0.1:{}/webdav/{}/", https_port, client_id);
    tracing::info!("Trying mount_webdav -S (HTTPS {}) ...", https_port);
    let mount_result2 = tokio::process::Command::new("mount_webdav")
        .args(["-S", &https_url, &mount_point])
        .output()
        .await;

    match &mount_result2 {
        Ok(output) if output.status.success() => {
            tracing::info!("mount_webdav HTTPS success: {}", mount_point);
            return Ok(Json(json!({
                "ok": true,
                "mount_point": mount_point,
                "webdav_url": https_url,
            })));
        }
        Ok(output) => {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let code = output.status.code().unwrap_or(-1);
            tracing::warn!("mount_webdav HTTPS failed (exit {}): {}", code, stderr);
        }
        Err(e) => {
            tracing::warn!("mount_webdav HTTPS exec error: {}", e);
        }
    }

    // Strategy 3: osascript (Finder "mount volume") via 127.0.0.1
    tracing::info!("Fallback: trying Finder mount volume ...");
    let applescript = format!(
        r#"tell application "Finder" to mount volume "{}""#,
        webdav_url
    );

    let finder_result = tokio::process::Command::new("osascript")
        .args(["-e", &applescript])
        .output()
        .await;

    match finder_result {
        Ok(output) if output.status.success() => {
            let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
            tracing::info!("Finder mount success: stdout={}", stdout);

            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

            let actual_mount = find_webdav_mount(client_id)
                .await
                .unwrap_or_else(|| mount_point.clone());

            Ok(Json(json!({
                "ok": true,
                "mount_point": actual_mount,
                "webdav_url": webdav_url,
            })))
        }
        Ok(output) => {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let exit_code = mount_result
                .as_ref()
                .ok()
                .and_then(|o| o.status.code())
                .unwrap_or(-1);
            tracing::error!("All mount strategies failed");
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": format!(
                        "マウント失敗 (mount_webdav exit {}, Finder: {})",
                        exit_code,
                        stderr.trim()
                    ),
                    "mount_point": mount_point,
                })),
            ))
        }
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("osascript実行エラー: {}", e)})),
        )),
    }
}

/// Find the actual mount point for a WebDAV client from system mount table.
async fn find_webdav_mount(client_id: &str) -> Option<String> {
    let output = tokio::process::Command::new("mount")
        .output()
        .await
        .ok()?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines() {
        if line.contains(client_id) || line.contains("/webdav/") {
            let parts: Vec<&str> = line.splitn(3, " on ").collect();
            if let Some(rest) = parts.get(1) {
                let mount_point = rest.split(" (").next().unwrap_or("").trim().to_string();
                if !mount_point.is_empty() {
                    return Some(mount_point);
                }
            }
        }
    }
    None
}

/// POST /api/unmount
/// Body: {"mount_path": "/Users/xxx/Public/mount/ljc-xxx"}
pub async fn unmount_webdav(
    State(state): State<Arc<AppState>>,
    Json(body): Json<UnmountRequest>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let mount_path = &body.mount_path;

    // Validate: path must contain "ljc-" prefix (our mount points)
    if !mount_path.contains("/ljc-") {
        return Err((
            StatusCode::FORBIDDEN,
            Json(json!({"error": "Can only unmount SnowSync mount points (ljc-*)"})),
        ));
    }

    // Validate: path must be under allowed mount base
    if let Err(e) = validate_mount_path(mount_path, &state.allowed_mount_base) {
        return Err((
            StatusCode::FORBIDDEN,
            Json(json!({"error": e})),
        ));
    }

    // Reject path traversal
    if mount_path.contains("..") {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Invalid path"})),
        ));
    }

    tracing::info!("Unmounting: {}", mount_path);

    let result = tokio::process::Command::new("umount")
        .arg(mount_path)
        .output()
        .await;

    match result {
        Ok(output) => {
            if output.status.success() {
                tracing::info!("Unmount success: {}", mount_path);
                Ok(Json(json!({
                    "ok": true,
                    "mount_path": mount_path,
                })))
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": format!("アンマウント失敗: {}", stderr)})),
                ))
            }
        }
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("umount実行エラー: {}", e)})),
        )),
    }
}

/// GET /api/mounts
/// Returns list of currently mounted WebDAV filesystems.
pub async fn list_mounts(
    State(state): State<Arc<AppState>>,
) -> Json<Value> {
    let port_str = state.port.to_string();
    let https_port_str = (state.port + 1).to_string();
    let result = tokio::process::Command::new("mount")
        .output()
        .await;

    match result {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let mounts: Vec<Value> = stdout
                .lines()
                .filter(|line| {
                    line.contains("webdav")
                        || line.contains(&port_str)
                        || line.contains(&https_port_str)
                        || line.contains("/webdav/")
                })
                .map(|line| {
                    let parts: Vec<&str> = line.splitn(3, " on ").collect();
                    let url = parts.first().unwrap_or(&"").trim().to_string();
                    let rest = parts.get(1).unwrap_or(&"").trim().to_string();
                    let mount_point = rest.split(" (").next().unwrap_or("").to_string();
                    json!({
                        "url": url,
                        "mount_point": mount_point,
                        "raw": line,
                    })
                })
                .collect();
            Json(json!(mounts))
        }
        Err(_) => Json(json!([])),
    }
}
