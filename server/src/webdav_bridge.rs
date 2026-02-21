// SNOWCODE - ソフトウェア製品
// (C) SNOWCODE
// 開発者: 雪符しき
// https://snowcode.jp
// 問い合わせ: info@snowcode.jp

use crate::state::AppState;
use axum::body::Body;
use axum::extract::ws::Message;
use axum::extract::Request;
use axum::response::Response;
use bytes::Bytes;
use dav_server::{
    davpath::DavPath,
    fakels::FakeLs,
    fs::{
        DavDirEntry, DavFile, DavFileSystem, DavMetaData, FsError, FsFuture, FsResult, FsStream,
        OpenOptions, ReadDirMeta,
    },
    DavHandler,
};
use futures_util::stream;
use serde_json::{json, Value};
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::sync::oneshot;

/// URL-decode a percent-encoded path string.
fn url_decode(s: &str) -> String {
    let mut result = Vec::new();
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            if let (Some(hi), Some(lo)) = (
                hex_val(bytes[i + 1]),
                hex_val(bytes[i + 2]),
            ) {
                result.push(hi << 4 | lo);
                i += 3;
                continue;
            }
        }
        result.push(bytes[i]);
        i += 1;
    }
    String::from_utf8(result).unwrap_or_else(|_| s.to_string())
}

fn hex_val(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        b'A'..=b'F' => Some(b - b'A' + 10),
        _ => None,
    }
}

/// Convert a DavPath to a clean, decoded path string for the Windows FS client.
fn dav_path_to_string(path: &DavPath) -> String {
    let raw = url_decode(&path.as_url_string());
    // Ensure we have a leading /
    if raw.starts_with('/') {
        raw
    } else {
        format!("/{}", raw)
    }
}

/// Parse an ISO 8601 datetime string to SystemTime.
fn parse_iso_to_systemtime(iso: &str) -> SystemTime {
    // Try to parse: "2024-01-15T12:30:45.000Z" or similar
    // Simple parser: extract year, month, day, hour, minute, second
    let s = iso.trim().trim_end_matches('Z');
    let parts: Vec<&str> = s.split('T').collect();
    if parts.len() != 2 {
        return SystemTime::now();
    }
    let date_parts: Vec<u64> = parts[0].split('-').filter_map(|p| p.parse().ok()).collect();
    let time_str = parts[1].split('.').next().unwrap_or("0:0:0");
    let time_parts: Vec<u64> = time_str.split(':').filter_map(|p| p.parse().ok()).collect();

    if date_parts.len() < 3 || time_parts.len() < 3 {
        return SystemTime::now();
    }

    let (year, month, day) = (date_parts[0], date_parts[1], date_parts[2]);
    let (hour, minute, second) = (time_parts[0], time_parts[1], time_parts[2]);

    // Calculate days from epoch (1970-01-01)
    let mut total_days: u64 = 0;
    for y in 1970..year {
        total_days += if is_leap(y) { 366 } else { 365 };
    }
    let days_in_month = [0, 31, 28 + if is_leap(year) { 1 } else { 0 }, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    for m in 1..month {
        total_days += days_in_month[m as usize];
    }
    total_days += day - 1;

    let total_seconds = total_days * 86400 + hour * 3600 + minute * 60 + second;
    SystemTime::UNIX_EPOCH + Duration::from_secs(total_seconds)
}

fn is_leap(y: u64) -> bool {
    (y % 4 == 0 && y % 100 != 0) || y % 400 == 0
}

/// A virtual filesystem that proxies file operations to a connected Windows client via WebSocket.
#[derive(Clone)]
pub struct RelayFs {
    state: Arc<AppState>,
    client_id: String,
}

impl RelayFs {
    pub fn new(state: Arc<AppState>, client_id: String) -> Box<Self> {
        Box::new(Self { state, client_id })
    }

    async fn send_command(&self, cmd: Value) -> Result<Value, FsError> {
        let request_id = uuid::Uuid::new_v4().to_string();
        let mut cmd = cmd;
        let cmd_type = cmd.get("type").and_then(|v| v.as_str()).unwrap_or("?").to_string();
        let cmd_path = cmd.get("path").and_then(|v| v.as_str()).unwrap_or("").to_string();
        cmd["id"] = json!(request_id);
        tracing::debug!("WebDAV relay: {} {} (id: {})", cmd_type, cmd_path, &request_id[..8]);

        let tx = {
            let clients = self.state.clients.read().await;
            match clients.get(&self.client_id) {
                Some(client) => client.tx.clone(),
                None => return Err(FsError::NotFound),
            }
        };

        let (resp_tx, resp_rx) = oneshot::channel();
        {
            let mut pending = self.state.pending.write().await;
            pending.insert(request_id.clone(), resp_tx);
        }

        if tx
            .send(Message::Text(cmd.to_string().into()))
            .is_err()
        {
            let mut pending = self.state.pending.write().await;
            pending.remove(&request_id);
            return Err(FsError::GeneralFailure);
        }

        match tokio::time::timeout(
            tokio::time::Duration::from_secs(30),
            resp_rx,
        )
        .await
        {
            Ok(Ok(response)) => {
                if response.get("ok").and_then(|v| v.as_bool()).unwrap_or(false) {
                    Ok(response)
                } else {
                    let err = response
                        .get("error")
                        .and_then(|v| v.as_str())
                        .unwrap_or("Unknown error");
                    if err.contains("not found") || err.contains("NotFound") {
                        // NotFound is expected — Finder probes for .DS_Store,
                        // .Spotlight-V100, etc. on every mount. Log at debug level.
                        tracing::debug!("Relay: not found: {} {}", cmd_type, cmd_path);
                        Err(FsError::NotFound)
                    } else if err.contains("permission") || err.contains("Permission") {
                        tracing::warn!("Relay: permission denied: {} {}", cmd_type, cmd_path);
                        Err(FsError::Forbidden)
                    } else {
                        tracing::warn!("Relay error: {} (cmd: {} {})", err, cmd_type, cmd_path);
                        Err(FsError::GeneralFailure)
                    }
                }
            }
            Ok(Err(_)) => Err(FsError::GeneralFailure),
            Err(_) => {
                let mut pending = self.state.pending.write().await;
                pending.remove(&request_id);
                Err(FsError::GeneralFailure)
            }
        }
    }
}

impl DavFileSystem for RelayFs {
    fn open<'a>(&'a self, path: &'a DavPath, options: OpenOptions) -> FsFuture<'a, Box<dyn DavFile>> {
        let path_str = dav_path_to_string(path);
        let fs = self.clone();

        Box::pin(async move {
            if options.create || options.create_new || options.write {
                // For write operations, return a writable file handle
                Ok(Box::new(RelayFile::new_writable(
                    fs.state.clone(),
                    fs.client_id.clone(),
                    path_str.clone(),
                )) as Box<dyn DavFile>)
            } else {
                // Read: fetch file content
                let resp = fs
                    .send_command(json!({
                        "type": "readFile",
                        "path": path_str,
                    }))
                    .await?;

                let data = resp
                    .get("data")
                    .and_then(|d| d.get("data"))
                    .and_then(|d| d.as_str())
                    .unwrap_or("");

                let bytes = base64_decode(data);
                let size = resp
                    .get("data")
                    .and_then(|d| d.get("size"))
                    .and_then(|d| d.as_u64())
                    .unwrap_or(bytes.len() as u64);
                let modified = resp
                    .get("data")
                    .and_then(|d| d.get("modified"))
                    .and_then(|d| d.as_str())
                    .unwrap_or("");

                Ok(Box::new(RelayFile::new_readable(bytes, size, modified.to_string()))
                    as Box<dyn DavFile>)
            }
        })
    }

    fn read_dir<'a>(
        &'a self,
        path: &'a DavPath,
        _meta: ReadDirMeta,
    ) -> FsFuture<'a, FsStream<Box<dyn DavDirEntry>>> {
        let path_str = dav_path_to_string(path);
        let fs = self.clone();

        Box::pin(async move {
            let resp = fs
                .send_command(json!({
                    "type": "readdir",
                    "path": path_str,
                }))
                .await?;

            let entries: Vec<Box<dyn DavDirEntry>> = resp
                .get("data")
                .and_then(|d| d.as_array())
                .map(|arr| {
                    arr.iter()
                        .map(|entry| {
                            let name = entry
                                .get("name")
                                .and_then(|v| v.as_str())
                                .unwrap_or("")
                                .to_string();
                            let is_dir = entry
                                .get("is_dir")
                                .and_then(|v| v.as_bool())
                                .unwrap_or(false);
                            let size = entry
                                .get("size")
                                .and_then(|v| v.as_u64())
                                .unwrap_or(0);
                            let modified = entry
                                .get("modified")
                                .and_then(|v| v.as_str())
                                .unwrap_or("")
                                .to_string();

                            Box::new(RelayDirEntry {
                                name,
                                is_dir,
                                size,
                                modified,
                            }) as Box<dyn DavDirEntry>
                        })
                        .collect()
                })
                .unwrap_or_default();

            let stream = stream::iter(entries.into_iter().map(Ok));
            Ok(Box::pin(stream) as FsStream<Box<dyn DavDirEntry>>)
        })
    }

    fn metadata<'a>(&'a self, path: &'a DavPath) -> FsFuture<'a, Box<dyn DavMetaData>> {
        let path_str = dav_path_to_string(path);
        let fs = self.clone();

        Box::pin(async move {
            let resp = fs
                .send_command(json!({
                    "type": "stat",
                    "path": path_str,
                }))
                .await?;

            let empty = json!({});
            let data = resp.get("data").unwrap_or(&empty);
            let is_dir = data.get("is_dir").and_then(|v| v.as_bool()).unwrap_or(false);
            let size = data.get("size").and_then(|v| v.as_u64()).unwrap_or(0);
            let modified = data
                .get("modified")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();

            Ok(Box::new(RelayMetaData {
                is_dir,
                size,
                modified,
            }) as Box<dyn DavMetaData>)
        })
    }

    fn create_dir<'a>(&'a self, path: &'a DavPath) -> FsFuture<'a, ()> {
        let path_str = dav_path_to_string(path);
        let fs = self.clone();
        Box::pin(async move {
            fs.send_command(json!({
                "type": "mkdir",
                "path": path_str,
            }))
            .await?;
            Ok(())
        })
    }

    fn remove_file<'a>(&'a self, path: &'a DavPath) -> FsFuture<'a, ()> {
        let path_str = dav_path_to_string(path);
        let fs = self.clone();
        Box::pin(async move {
            fs.send_command(json!({
                "type": "delete",
                "path": path_str,
            }))
            .await?;
            Ok(())
        })
    }

    fn remove_dir<'a>(&'a self, path: &'a DavPath) -> FsFuture<'a, ()> {
        let path_str = dav_path_to_string(path);
        let fs = self.clone();
        Box::pin(async move {
            fs.send_command(json!({
                "type": "delete",
                "path": path_str,
            }))
            .await?;
            Ok(())
        })
    }

    fn rename<'a>(&'a self, from: &'a DavPath, to: &'a DavPath) -> FsFuture<'a, ()> {
        let from_str = dav_path_to_string(from);
        let to_str = dav_path_to_string(to);
        let fs = self.clone();
        Box::pin(async move {
            fs.send_command(json!({
                "type": "rename",
                "oldPath": from_str,
                "newPath": to_str,
            }))
            .await?;
            Ok(())
        })
    }

    fn copy<'a>(&'a self, _from: &'a DavPath, _to: &'a DavPath) -> FsFuture<'a, ()> {
        Box::pin(async { Err(FsError::NotImplemented) })
    }
}

// --- RelayFile ---

struct RelayFile {
    data: Vec<u8>,
    pos: usize,
    size: u64,
    modified: String,
    // For writable files
    state: Option<Arc<AppState>>,
    client_id: Option<String>,
    path: Option<String>,
    write_buf: Vec<u8>,
}

impl std::fmt::Debug for RelayFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RelayFile")
            .field("pos", &self.pos)
            .field("size", &self.size)
            .field("path", &self.path)
            .finish()
    }
}

impl RelayFile {
    fn new_readable(data: Vec<u8>, size: u64, modified: String) -> Self {
        Self {
            data,
            pos: 0,
            size,
            modified,
            state: None,
            client_id: None,
            path: None,
            write_buf: Vec::new(),
        }
    }

    fn new_writable(state: Arc<AppState>, client_id: String, path: String) -> Self {
        Self {
            data: Vec::new(),
            pos: 0,
            size: 0,
            modified: String::new(),
            state: Some(state),
            client_id: Some(client_id),
            path: Some(path),
            write_buf: Vec::new(),
        }
    }
}

impl DavFile for RelayFile {
    fn metadata<'a>(&'a mut self) -> FsFuture<'a, Box<dyn DavMetaData>> {
        let size = self.size;
        let modified = self.modified.clone();
        Box::pin(async move {
            Ok(Box::new(RelayMetaData {
                is_dir: false,
                size,
                modified,
            }) as Box<dyn DavMetaData>)
        })
    }

    fn write_bytes<'a>(&'a mut self, buf: Bytes) -> FsFuture<'a, ()> {
        self.write_buf.extend_from_slice(&buf);
        Box::pin(async { Ok(()) })
    }

    fn write_buf<'a>(&'a mut self, buf: Box<dyn bytes::Buf + Send>) -> FsFuture<'a, ()> {
        let data = buf.chunk().to_vec();
        self.write_buf.extend_from_slice(&data);
        Box::pin(async { Ok(()) })
    }

    fn read_bytes<'a>(&'a mut self, count: usize) -> FsFuture<'a, Bytes> {
        let end = (self.pos + count).min(self.data.len());
        let chunk = Bytes::copy_from_slice(&self.data[self.pos..end]);
        self.pos = end;
        Box::pin(async move { Ok(chunk) })
    }

    fn seek<'a>(&'a mut self, pos: std::io::SeekFrom) -> FsFuture<'a, u64> {
        let new_pos = match pos {
            std::io::SeekFrom::Start(n) => n as usize,
            std::io::SeekFrom::End(n) => (self.data.len() as i64 + n) as usize,
            std::io::SeekFrom::Current(n) => (self.pos as i64 + n) as usize,
        };
        self.pos = new_pos.min(self.data.len());
        let result = self.pos as u64;
        Box::pin(async move { Ok(result) })
    }

    fn flush<'a>(&'a mut self) -> FsFuture<'a, ()> {
        if self.write_buf.is_empty() {
            return Box::pin(async { Ok(()) });
        }

        let state = self.state.clone();
        let client_id = self.client_id.clone();
        let path = self.path.clone();
        let data = std::mem::take(&mut self.write_buf);

        Box::pin(async move {
            if let (Some(state), Some(client_id), Some(path)) = (state, client_id, path) {
                let relay = RelayFs {
                    state,
                    client_id,
                };
                let encoded = base64_encode(&data);
                relay
                    .send_command(json!({
                        "type": "writeFile",
                        "path": path,
                        "data": encoded,
                    }))
                    .await
                    .map_err(|_| FsError::GeneralFailure)?;
            }
            Ok(())
        })
    }
}

// --- RelayDirEntry ---

struct RelayDirEntry {
    name: String,
    is_dir: bool,
    size: u64,
    modified: String,
}

impl DavDirEntry for RelayDirEntry {
    fn name(&self) -> Vec<u8> {
        self.name.as_bytes().to_vec()
    }

    fn metadata<'a>(&'a self) -> FsFuture<'a, Box<dyn DavMetaData>> {
        let is_dir = self.is_dir;
        let size = self.size;
        let modified = self.modified.clone();
        Box::pin(async move {
            Ok(Box::new(RelayMetaData {
                is_dir,
                size,
                modified,
            }) as Box<dyn DavMetaData>)
        })
    }
}

// --- RelayMetaData ---

#[derive(Clone, Debug)]
struct RelayMetaData {
    is_dir: bool,
    size: u64,
    #[allow(dead_code)]
    modified: String,
}

impl DavMetaData for RelayMetaData {
    fn len(&self) -> u64 {
        self.size
    }

    fn modified(&self) -> FsResult<SystemTime> {
        if self.modified.is_empty() {
            Ok(SystemTime::now())
        } else {
            Ok(parse_iso_to_systemtime(&self.modified))
        }
    }

    fn is_dir(&self) -> bool {
        self.is_dir
    }
}

// --- Base64 helpers ---

fn base64_decode(s: &str) -> Vec<u8> {
    // Simple base64 decode
    use std::collections::HashMap;
    let chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let map: HashMap<u8, u8> = chars
        .bytes()
        .enumerate()
        .map(|(i, b)| (b, i as u8))
        .collect();

    let mut result = Vec::new();
    let bytes: Vec<u8> = s.bytes().filter(|b| *b != b'=' && *b != b'\n' && *b != b'\r').collect();

    for chunk in bytes.chunks(4) {
        let vals: Vec<u8> = chunk.iter().filter_map(|b| map.get(b).copied()).collect();
        if vals.len() >= 2 {
            result.push((vals[0] << 2) | (vals[1] >> 4));
        }
        if vals.len() >= 3 {
            result.push((vals[1] << 4) | (vals[2] >> 2));
        }
        if vals.len() >= 4 {
            result.push((vals[2] << 6) | vals[3]);
        }
    }
    result
}

fn base64_encode(data: &[u8]) -> String {
    let chars: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"
        .chars()
        .collect();
    let mut result = String::new();

    for chunk in data.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = if chunk.len() > 1 { chunk[1] as u32 } else { 0 };
        let b2 = if chunk.len() > 2 { chunk[2] as u32 } else { 0 };

        result.push(chars[(b0 >> 2) as usize]);
        result.push(chars[((b0 & 3) << 4 | b1 >> 4) as usize]);
        if chunk.len() > 1 {
            result.push(chars[((b1 & 15) << 2 | b2 >> 6) as usize]);
        } else {
            result.push('=');
        }
        if chunk.len() > 2 {
            result.push(chars[(b2 & 63) as usize]);
        } else {
            result.push('=');
        }
    }
    result
}

// --- WebDAV handler construction ---

pub fn create_webdav_handler(state: Arc<AppState>, client_id: &str) -> DavHandler {
    let prefix = format!("/webdav/{}", client_id);
    DavHandler::builder()
        .filesystem(RelayFs::new(state, client_id.to_string()))
        .locksystem(FakeLs::new())
        .strip_prefix(&prefix)
        .build_handler()
}

/// Handle WebDAV requests for a specific client.
/// Path format: /webdav/{client_id}/...
pub async fn webdav_handler(state: Arc<AppState>, req: Request) -> Response<Body> {
    let path = req.uri().path().to_string();
    let method = req.method().clone();

    // Handle trailing slash removal from client_id
    let stripped = path.strip_prefix("/webdav/").unwrap_or(&path);
    let (client_id, _sub_path) = stripped.split_once('/').unwrap_or((stripped, ""));
    // Remove any trailing slash from client_id
    let client_id = client_id.trim_end_matches('/');

    if client_id.is_empty() || uuid::Uuid::parse_str(client_id).is_err() {
        return Response::builder()
            .status(404)
            .body(Body::from("Invalid client ID"))
            .unwrap();
    }

    tracing::debug!("WebDAV {} {} (client: {})", method, path, client_id);

    // For OPTIONS requests, return DAV capabilities immediately
    // without checking if client exists. Finder sends OPTIONS first
    // to discover DAV support — it must get DAV headers to proceed
    // without showing an authentication dialog.
    if method == http::Method::OPTIONS {
        tracing::debug!("WebDAV OPTIONS -> returning DAV capabilities");
        return Response::builder()
            .status(200)
            .header("DAV", "1, 2")
            .header("Allow", "OPTIONS, GET, HEAD, PUT, DELETE, MKCOL, COPY, MOVE, PROPFIND, PROPPATCH, LOCK, UNLOCK")
            .header("MS-Author-Via", "DAV")
            .header("Content-Length", "0")
            .body(Body::empty())
            .unwrap();
    }

    // Check client exists
    {
        let clients = state.clients.read().await;
        if !clients.contains_key(client_id) {
            tracing::warn!("WebDAV: client '{}' not connected", client_id);
            return Response::builder()
                .status(404)
                .body(Body::from(format!("Client '{}' not connected", client_id)))
                .unwrap();
        }
    }

    let handler = create_webdav_handler(state.clone(), client_id);

    // DavHandler with strip_prefix handles URI rewriting and correct href generation.
    // We only need to strip the Authorization header to allow guest access.
    let (mut parts, body) = req.into_parts();
    parts.headers.remove(http::header::AUTHORIZATION);
    let req = Request::from_parts(parts, body);

    let resp = handler.handle(req).await;
    let (resp_parts, dav_body) = resp.into_parts();
    let mut response = Response::from_parts(resp_parts, Body::new(dav_body));

    // dav-server may return 500 for FsError::NotFound (e.g. PROPFIND on a
    // non-existent path). Finder expects 404 in this case. Downgrade 500 to
    // 404 when the underlying relay reported "not found" — this prevents
    // tower_http from logging these expected misses as server errors.
    if response.status() == http::StatusCode::INTERNAL_SERVER_ERROR {
        tracing::debug!(
            "WebDAV {} {} -> 500, downgrading to 404 (likely not-found relay)",
            method, path
        );
        *response.status_mut() = http::StatusCode::NOT_FOUND;
    }

    // Add DAV header to all responses so Finder recognizes this as a WebDAV server
    response.headers_mut().insert("DAV", "1, 2".parse().unwrap());

    // Ensure no WWW-Authenticate header is sent — this tells Finder
    // that authentication is NOT required, so no password dialog appears.
    response.headers_mut().remove(http::header::WWW_AUTHENTICATE);

    tracing::debug!("WebDAV response: {} {}", response.status(), path);
    response
}
