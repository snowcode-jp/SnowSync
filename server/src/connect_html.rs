// SNOWCODE - ソフトウェア製品
// (C) SNOWCODE
// 開発者: 雪符しき
// https://snowcode.jp
// 問い合わせ: info@snowcode.jp

use axum::extract::{Query, State};
use axum::http::header;
use axum::http::HeaderMap;
use axum::response::{Html, IntoResponse};
use serde::Deserialize;
use std::sync::Arc;

use crate::state::AppState;

#[derive(Deserialize)]
pub struct ConnectParams {
    pub ip: Option<String>,
    pub port: Option<u16>,
}

/// GET /api/connect-html?ip=...&port=...
/// Downloads the standalone connect HTML file with IP pre-filled and auth token embedded.
/// If no ip param, uses the Host header (= the IP the client used to reach this server).
fn extract_host_ip(headers: &HeaderMap) -> String {
    headers
        .get(header::HOST)
        .and_then(|v| v.to_str().ok())
        .map(|h| h.split(':').next().unwrap_or("").to_string())
        .unwrap_or_default()
}

fn extract_host_port(headers: &HeaderMap) -> u16 {
    headers
        .get(header::HOST)
        .and_then(|v| v.to_str().ok())
        .and_then(|h| h.split(':').nth(1))
        .and_then(|p: &str| p.parse::<u16>().ok())
        .unwrap_or(17200)
}

/// Escape a string for safe embedding inside an HTML attribute value.
fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('"', "&quot;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('\'', "&#x27;")
}

pub async fn connect_html(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Query(params): Query<ConnectParams>,
) -> impl IntoResponse {
    let ip = html_escape(&params.ip.unwrap_or_else(|| extract_host_ip(&headers)));
    let port = params.port.unwrap_or_else(|| extract_host_port(&headers));
    let token = &state.api_token;
    let html = generate_connect_html(&ip, port, token);

    (
        [
            (header::CONTENT_TYPE, "text/html; charset=utf-8"),
            (header::CONTENT_DISPOSITION, "attachment; filename=\"SnowSync-Connect.html\""),
        ],
        html,
    )
}

/// GET / — Download landing page (shown in browser, not downloaded)
pub async fn download_page(headers: HeaderMap) -> Html<String> {
    let ip = extract_host_ip(&headers);
    let port = extract_host_port(&headers);
    Html(generate_download_page(&ip, port))
}

fn generate_download_page(server_ip: &str, port: u16) -> String {
    format!(r##"<!DOCTYPE html>
<html lang="ja">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>SnowSync - クライアント接続</title>
<link rel="preconnect" href="https://fonts.googleapis.com">
<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
<link href="https://fonts.googleapis.com/css2?family=Zen+Maru+Gothic:wght@400;500;700&display=swap" rel="stylesheet">
<style>
  * {{ margin: 0; padding: 0; box-sizing: border-box; }}
  body {{
    font-family: 'Zen Maru Gothic', 'Helvetica Neue', Arial, 'Hiragino Kaku Gothic ProN', 'Hiragino Sans', Meiryo, sans-serif;
    background: linear-gradient(135deg, #e8f4fc 0%, #f0f8ff 50%, #e6f3fa 100%);
    min-height: 100vh; color: #4a6b7c;
    display: flex; flex-direction: column; align-items: center;
    justify-content: center; padding: 40px 20px; position: relative;
  }}
  body::before {{
    content: ''; position: fixed; top: 0; left: 0; right: 0; bottom: 0;
    background-image:
      radial-gradient(circle at 10% 20%, rgba(126,184,216,0.08) 0%, transparent 50%),
      radial-gradient(circle at 90% 80%, rgba(126,184,216,0.08) 0%, transparent 50%);
    pointer-events: none; z-index: 0;
  }}
  .container {{ max-width: 520px; width: 100%; position: relative; z-index: 1; text-align: center; }}
  .page-header {{
    display: flex; align-items: center; justify-content: center; gap: 12px;
    margin-bottom: 8px; font-size: 28px; font-weight: 700; color: #4a7c9b;
  }}
  .snowflake {{ color: #7eb8d8; }}
  .subtitle {{ color: #7eb8d8; font-size: 14px; margin-bottom: 36px; font-weight: 500; }}
  .card {{
    background: rgba(255,255,255,0.9); backdrop-filter: blur(10px);
    border-radius: 20px; padding: 36px; margin-bottom: 20px;
    box-shadow: 0 4px 20px rgba(100,150,200,0.1), inset 0 1px 0 rgba(255,255,255,0.8);
    border: 1px solid rgba(126,184,216,0.2); position: relative; overflow: hidden;
  }}
  .card::before {{
    content: ''; position: absolute; top: 0; right: 0; width: 120px; height: 120px;
    background: radial-gradient(circle at top right, rgba(126,184,216,0.1) 0%, transparent 70%);
    pointer-events: none;
  }}
  .download-icon {{
    font-size: 48px; margin-bottom: 16px; color: #7eb8d8;
  }}
  .card h2 {{
    font-size: 18px; color: #4a7c9b; margin-bottom: 12px; font-weight: 700;
  }}
  .card p {{
    font-size: 14px; color: #6a8fa5; line-height: 1.8; margin-bottom: 24px;
  }}
  .btn {{
    display: inline-flex; align-items: center; gap: 10px;
    padding: 16px 36px; border: none; border-radius: 30px; cursor: pointer;
    font-size: 16px; font-weight: 700; font-family: inherit;
    text-decoration: none; transition: all 0.3s ease; position: relative; overflow: hidden;
    background: linear-gradient(135deg, #7eb8d8 0%, #5a9fc8 100%);
    color: #fff; box-shadow: 0 4px 15px rgba(90,159,200,0.3);
  }}
  .btn:hover {{
    transform: translateY(-2px);
    box-shadow: 0 6px 20px rgba(90,159,200,0.4);
  }}
  .btn::before {{
    content: ''; position: absolute; top: 0; left: -100%; width: 100%; height: 100%;
    background: linear-gradient(90deg, transparent, rgba(255,255,255,0.3), transparent);
    transition: left 0.5s ease;
  }}
  .btn:hover::before {{ left: 100%; }}
  .steps {{
    text-align: left; margin-top: 28px;
  }}
  .step {{
    display: flex; align-items: flex-start; gap: 14px; margin-bottom: 16px;
  }}
  .step-num {{
    width: 28px; height: 28px; border-radius: 50%; flex-shrink: 0;
    background: linear-gradient(135deg, #7eb8d8 0%, #5a9fc8 100%);
    color: #fff; display: flex; align-items: center; justify-content: center;
    font-size: 13px; font-weight: 700;
  }}
  .step-text {{ font-size: 13px; color: #4a6b7c; line-height: 1.6; padding-top: 3px; }}
  .step-text strong {{ color: #4a7c9b; }}
  .note {{
    background: rgba(126,184,216,0.08); border-radius: 12px; padding: 16px;
    border: 1px solid rgba(126,184,216,0.15); margin-top: 20px;
    font-size: 12px; color: #6a8fa5; text-align: left; line-height: 1.7;
  }}
  .note strong {{ color: #4a7c9b; }}
  .footer {{ margin-top: 20px; font-size: 12px; color: #a8c8dc; }}
</style>
</head>
<body>
<div class="container">
  <div class="page-header">
    <span class="snowflake">&#10052;</span>
    SnowSync
  </div>
  <p class="subtitle">Windows PC &#x2194; Mac ファイル共有クライアント</p>

  <div class="card">
    <div class="download-icon">&#128229;</div>
    <h2>接続HTMLをダウンロード</h2>
    <p>
      ダウンロードしたHTMLファイルをChromeまたはEdgeで開き、<br>
      共有するフォルダを選択してください。
    </p>

    <a class="btn" href="/api/connect-html" download="SnowSync-Connect.html">
      &#10052; ダウンロード
    </a>

    <div class="steps">
      <div class="step">
        <div class="step-num">1</div>
        <div class="step-text">上のボタンで <strong>SnowSync-Connect.html</strong> をダウンロード</div>
      </div>
      <div class="step">
        <div class="step-num">2</div>
        <div class="step-text">ダウンロードしたファイルを <strong>Chrome</strong> または <strong>Edge</strong> で開く</div>
      </div>
      <div class="step">
        <div class="step-num">3</div>
        <div class="step-text"><strong>フォルダを選択して接続</strong>ボタンを押す</div>
      </div>
    </div>

    <div class="note">
      <strong>&#9432; なぜダウンロードが必要？</strong><br>
      ファイル操作にはブラウザの File System Access API を使用しています。
      セキュリティ上、この API は <strong>file://</strong> または <strong>https://</strong> でのみ動作します。
      サーバーIP (<strong>{server_ip}:{port}</strong>) は自動で設定済みです。
    </div>
  </div>

  <p class="footer">&#10052; SnowSync v1.0 &mdash; SNOWCODE</p>
</div>
</body>
</html>"##, server_ip = server_ip, port = port)
}

fn generate_connect_html(server_ip: &str, port: u16, api_token: &str) -> String {
    format!(r##"<!DOCTYPE html>
<html lang="ja">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>SnowSync - 接続クライアント</title>
<link rel="preconnect" href="https://fonts.googleapis.com">
<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
<link href="https://fonts.googleapis.com/css2?family=Zen+Maru+Gothic:wght@400;500;700&display=swap" rel="stylesheet">
<style>
  * {{ margin: 0; padding: 0; box-sizing: border-box; }}
  body {{
    font-family: 'Zen Maru Gothic', 'Helvetica Neue', Arial, 'Hiragino Kaku Gothic ProN', 'Hiragino Sans', Meiryo, sans-serif;
    background: linear-gradient(135deg, #e8f4fc 0%, #f0f8ff 50%, #e6f3fa 100%);
    min-height: 100vh; color: #4a6b7c;
    display: flex; flex-direction: column; align-items: center;
    padding: 40px 20px; position: relative;
  }}
  body::before {{
    content: ''; position: fixed; top: 0; left: 0; right: 0; bottom: 0;
    background-image:
      radial-gradient(circle at 10% 20%, rgba(126,184,216,0.08) 0%, transparent 50%),
      radial-gradient(circle at 90% 80%, rgba(126,184,216,0.08) 0%, transparent 50%),
      radial-gradient(circle at 50% 50%, rgba(200,220,240,0.05) 0%, transparent 70%);
    pointer-events: none; z-index: 0;
  }}
  .container {{ max-width: 640px; width: 100%; position: relative; z-index: 1; }}

  /* ヘッダー */
  .page-header {{
    display: flex; align-items: center; gap: 12px;
    margin-bottom: 8px; font-size: 24px; font-weight: 700; color: #4a7c9b;
  }}
  .page-header .snowflake {{ color: #7eb8d8; font-size: 22px; }}
  .subtitle {{ color: #7eb8d8; font-size: 14px; margin-bottom: 30px; font-weight: 500; }}

  /* カード */
  .card {{
    background: rgba(255,255,255,0.9); backdrop-filter: blur(10px);
    border-radius: 20px; padding: 25px; margin-bottom: 20px;
    box-shadow: 0 4px 20px rgba(100,150,200,0.1), inset 0 1px 0 rgba(255,255,255,0.8);
    border: 1px solid rgba(126,184,216,0.2); position: relative; overflow: hidden;
  }}
  .card::before {{
    content: ''; position: absolute; top: 0; right: 0; width: 100px; height: 100px;
    background: radial-gradient(circle at top right, rgba(126,184,216,0.1) 0%, transparent 70%);
    pointer-events: none;
  }}
  .card-header {{
    display: flex; justify-content: space-between; align-items: center;
    margin-bottom: 20px; padding-bottom: 15px;
    border-bottom: 2px solid rgba(126,184,216,0.15);
  }}
  .card-title {{
    font-size: 16px; font-weight: 700; color: #4a7c9b;
    display: flex; align-items: center; gap: 10px;
  }}
  .card-title .icon {{ color: #7eb8d8; }}

  /* フォーム */
  .form-label {{
    display: block; margin-bottom: 8px; font-weight: 600;
    font-size: 13px; color: #4a7c9b;
  }}
  .form-input {{
    width: 100%; padding: 14px 18px;
    border: 2px solid rgba(126,184,216,0.3); border-radius: 12px;
    font-size: 14px; font-family: inherit;
    transition: all 0.3s ease;
    background: rgba(255,255,255,0.8); color: #4a6b7c;
    margin-bottom: 16px;
  }}
  .form-input:focus {{
    outline: none; border-color: #7eb8d8;
    box-shadow: 0 0 0 4px rgba(126,184,216,0.15); background: #fff;
  }}
  .form-input::placeholder {{ color: #a8c8dc; }}

  /* ステータス */
  .status-row {{
    display: flex; align-items: center; gap: 10px;
    margin-bottom: 20px; padding: 10px 16px;
    border-radius: 12px;
  }}
  .status-disconnected {{
    background: rgba(158,158,158,0.1); border: 1px solid rgba(158,158,158,0.2);
  }}
  .status-connecting {{
    background: linear-gradient(135deg, rgba(255,183,77,0.2) 0%, rgba(255,183,77,0.1) 100%);
    border: 1px solid rgba(230,81,0,0.2);
  }}
  .status-connected {{
    background: linear-gradient(135deg, rgba(102,187,106,0.2) 0%, rgba(102,187,106,0.1) 100%);
    border: 1px solid rgba(56,142,60,0.2);
  }}
  .dot {{
    width: 10px; height: 10px; border-radius: 50%;
    transition: background 0.3s; flex-shrink: 0;
  }}
  .dot-off {{ background: #bbb; }}
  .dot-connecting {{ background: #ef6c00; animation: pulse 1.5s infinite; }}
  .dot-on {{ background: #43a047; }}
  @keyframes pulse {{ 0%,100%{{opacity:1}} 50%{{opacity:0.3}} }}
  .status-text {{ font-size: 13px; font-weight: 600; }}

  /* ボタン */
  .btn {{
    display: inline-flex; align-items: center; gap: 8px;
    padding: 14px 28px; border: none; border-radius: 25px; cursor: pointer;
    font-size: 14px; font-weight: 600; font-family: inherit;
    transition: all 0.3s ease; position: relative; overflow: hidden;
  }}
  .btn::before {{
    content: ''; position: absolute; top: 0; left: -100%; width: 100%; height: 100%;
    background: linear-gradient(90deg, transparent, rgba(255,255,255,0.3), transparent);
    transition: left 0.5s ease;
  }}
  .btn:hover::before {{ left: 100%; }}
  .btn-primary {{
    background: linear-gradient(135deg, #7eb8d8 0%, #5a9fc8 100%);
    color: #fff; box-shadow: 0 4px 15px rgba(90,159,200,0.3);
  }}
  .btn-primary:hover {{
    transform: translateY(-2px);
    box-shadow: 0 6px 20px rgba(90,159,200,0.4);
  }}
  .btn-primary:disabled {{
    opacity: 0.5; cursor: not-allowed; transform: none;
  }}
  .btn-danger {{
    background: linear-gradient(135deg, #ff8a9b 0%, #ff6b7a 100%);
    color: #fff; box-shadow: 0 4px 15px rgba(255,107,122,0.3);
  }}
  .btn-danger:hover {{
    transform: translateY(-2px);
    box-shadow: 0 6px 20px rgba(255,107,122,0.4);
  }}

  /* 情報グリッド */
  .info-grid {{
    display: grid; grid-template-columns: 1fr 1fr; gap: 16px; margin-bottom: 16px;
  }}
  .info-item {{
    background: rgba(126,184,216,0.08); border-radius: 12px; padding: 14px;
    border: 1px solid rgba(126,184,216,0.12);
  }}
  .info-label {{
    font-size: 11px; color: #7eb8d8; text-transform: uppercase;
    letter-spacing: 1px; font-weight: 700; margin-bottom: 6px;
  }}
  .info-value {{
    font-size: 13px; color: #4a6b7c; font-weight: 500; word-break: break-all;
  }}

  /* アラート */
  .alert {{
    padding: 16px 20px; border-radius: 12px; margin-bottom: 20px;
    display: flex; align-items: flex-start; gap: 12px; font-size: 13px;
  }}
  .alert-error {{
    background: linear-gradient(135deg, rgba(255,138,155,0.3) 0%, rgba(255,138,155,0.1) 100%);
    color: #c62828; border: 1px solid rgba(198,40,40,0.2);
  }}

  /* ログ */
  .log-box {{
    font-family: 'SF Mono', Monaco, 'Cascadia Code', monospace;
    font-size: 11px; color: #7eb8d8;
    max-height: 180px; overflow-y: auto; padding: 4px 0;
  }}
  .log-box p {{ margin: 3px 0; line-height: 1.6; }}
  .log-box p:last-child {{ color: #4a7c9b; font-weight: 500; }}

  /* トースト */
  .toast-container {{
    position: fixed; bottom: 24px; right: 24px; z-index: 9999;
    display: flex; flex-direction: column-reverse; gap: 12px;
    max-width: 380px; pointer-events: none;
  }}
  .toast {{
    display: flex; align-items: flex-start; gap: 12px;
    padding: 16px 20px; border-radius: 16px;
    background: rgba(255,255,255,0.95); backdrop-filter: blur(12px);
    box-shadow: 0 8px 30px rgba(0,0,0,0.12), 0 2px 8px rgba(0,0,0,0.06),
      inset 0 1px 0 rgba(255,255,255,0.9);
    border: 1px solid rgba(126,184,216,0.2);
    pointer-events: auto;
    animation: toast-in 0.35s cubic-bezier(0.34,1.56,0.64,1);
  }}
  .toast-exit {{ animation: toast-out 0.3s ease-in forwards; }}
  @keyframes toast-in {{
    from {{ opacity:0; transform:translateX(80px) scale(0.8); }}
    to {{ opacity:1; transform:translateX(0) scale(1); }}
  }}
  @keyframes toast-out {{
    from {{ opacity:1; transform:translateX(0) scale(1); }}
    to {{ opacity:0; transform:translateX(80px) scale(0.8); }}
  }}
  .toast-success {{ border-left: 4px solid #66bb6a; }}
  .toast-success .toast-icon {{ color: #43a047; }}
  .toast-error {{ border-left: 4px solid #ff6b7a; }}
  .toast-error .toast-icon {{ color: #e53935; }}
  .toast-info {{ border-left: 4px solid #7eb8d8; }}
  .toast-info .toast-icon {{ color: #1976d2; }}
  .toast-icon {{ font-size: 18px; flex-shrink: 0; margin-top: 1px; }}
  .toast-message {{ flex: 1; font-size: 13px; font-weight: 500; line-height: 1.5; color: #4a6b7c; }}
  .toast-close {{
    background: none; border: none; cursor: pointer; color: #a8c8dc;
    font-size: 14px; padding: 2px 4px; border-radius: 6px;
    transition: all 0.2s ease; flex-shrink: 0;
  }}
  .toast-close:hover {{ color: #4a7c9b; background: rgba(126,184,216,0.15); }}

  /* スクロールバー */
  ::-webkit-scrollbar {{ width: 6px; height: 6px; }}
  ::-webkit-scrollbar-track {{ background: rgba(126,184,216,0.1); }}
  ::-webkit-scrollbar-thumb {{ background: rgba(126,184,216,0.3); border-radius: 3px; }}
  ::-webkit-scrollbar-thumb:hover {{ background: rgba(126,184,216,0.5); }}

  .hidden {{ display: none; }}
</style>
</head>
<body>
<div class="container">
  <div class="page-header">
    <span class="snowflake">&#10052;</span>
    SnowSync に接続
  </div>
  <p class="subtitle">このPCのフォルダをMacサーバーに共有します</p>

  <div id="error-box" class="alert alert-error hidden"></div>

  <!-- 接続設定カード -->
  <div class="card">
    <div class="card-header">
      <div class="card-title">
        <span class="icon">&#128268;</span>
        接続設定
      </div>
    </div>

    <label class="form-label">Mac サーバーの IP アドレス</label>
    <input id="server-ip" class="form-input" type="text" value="{server_ip}" placeholder="192.168.x.x" />

    <label class="form-label">このPCの名前</label>
    <input id="pc-name" class="form-input" type="text" value="" placeholder="例: デスクトップPC、ノートPC" />

    <div id="status-row" class="status-row status-disconnected">
      <div id="status-dot" class="dot dot-off"></div>
      <span id="status-text" class="status-text">未接続</span>
    </div>

    <div id="connect-section">
      <button id="btn-connect" class="btn btn-primary" onclick="handleConnect()">
        &#10052; フォルダを選択して接続
      </button>
    </div>

    <div id="connected-section" class="hidden">
      <div class="info-grid">
        <div class="info-item">
          <div class="info-label">PC名</div>
          <div id="connected-name" class="info-value">-</div>
        </div>
        <div class="info-item">
          <div class="info-label">共有フォルダ</div>
          <div id="folder-name" class="info-value">-</div>
        </div>
      </div>
      <div class="info-grid">
        <div class="info-item">
          <div class="info-label">クライアントID</div>
          <div id="client-id" class="info-value" style="font-size:11px;">-</div>
        </div>
        <div class="info-item">
          <div class="info-label">WebDAV URL</div>
          <div id="webdav-url" class="info-value" style="font-size:11px;">-</div>
        </div>
      </div>
      <button class="btn btn-danger" onclick="handleDisconnect()">&#x2716; 切断</button>
    </div>
  </div>

  <!-- ログカード -->
  <div class="card">
    <div class="card-header">
      <div class="card-title">
        <span class="icon">&#128220;</span>
        ログ
      </div>
    </div>
    <div id="log-box" class="log-box">
      <p>&#10052; 接続待機中...</p>
    </div>
  </div>
</div>

<!-- トースト -->
<div id="toast-container" class="toast-container"></div>

<script>
let ws = null;
let dirHandle = null;
let clientId = '';
let folderName = '';

function log(msg) {{
  const box = document.getElementById('log-box');
  const p = document.createElement('p');
  p.textContent = '[' + new Date().toLocaleTimeString() + '] ' + msg;
  box.appendChild(p);
  box.scrollTop = box.scrollHeight;
  while (box.children.length > 100) box.removeChild(box.firstChild);
}}

function setStatus(s) {{
  const dot = document.getElementById('status-dot');
  const text = document.getElementById('status-text');
  const row = document.getElementById('status-row');
  const connectSec = document.getElementById('connect-section');
  const connectedSec = document.getElementById('connected-section');
  const cls = {{ disconnected:'dot-off', connecting:'dot-connecting', connected:'dot-on' }};
  const rowCls = {{ disconnected:'status-disconnected', connecting:'status-connecting', connected:'status-connected' }};
  const labels = {{ disconnected:'未接続', connecting:'接続処理中...', connected:'❄ 接続中' }};
  dot.className = 'dot ' + (cls[s] || 'dot-off');
  row.className = 'status-row ' + (rowCls[s] || 'status-disconnected');
  text.textContent = labels[s] || '';
  if (s === 'connected') {{
    connectSec.classList.add('hidden');
    connectedSec.classList.remove('hidden');
    showToast('success', 'サーバーに接続しました');
  }} else if (s === 'disconnected' && clientId) {{
    connectSec.classList.remove('hidden');
    connectedSec.classList.add('hidden');
    showToast('info', '切断しました');
  }} else {{
    connectSec.classList.remove('hidden');
    connectedSec.classList.add('hidden');
  }}
}}

function showToast(type, message) {{
  const container = document.getElementById('toast-container');
  const icons = {{ success:'&#10004;', error:'&#10008;', info:'&#10052;' }};
  const toast = document.createElement('div');
  toast.className = 'toast toast-' + type;
  toast.innerHTML = '<span class="toast-icon">' + (icons[type]||'') + '</span>'
    + '<span class="toast-message">' + message + '</span>'
    + '<button class="toast-close" onclick="this.parentElement.classList.add(\'toast-exit\');setTimeout(()=>this.parentElement.remove(),300)">&times;</button>';
  container.appendChild(toast);
  setTimeout(() => {{ toast.classList.add('toast-exit'); setTimeout(() => toast.remove(), 300); }}, 4000);
}}

function showError(msg) {{
  const box = document.getElementById('error-box');
  if (msg) {{ box.textContent = msg; box.classList.remove('hidden'); showToast('error', msg); }}
  else {{ box.classList.add('hidden'); }}
}}

async function resolvePath(path) {{
  const segments = path.split('/').filter(Boolean);
  if (segments.length === 0) return {{ parent: dirHandle, name: '', segments: [] }};
  let current = dirHandle;
  for (let i = 0; i < segments.length - 1; i++) {{
    current = await current.getDirectoryHandle(segments[i]);
  }}
  return {{ parent: current, name: segments[segments.length - 1], segments }};
}}

async function handleCommand(cmd) {{
  const type = cmd.type;
  const path = cmd.path || '/';
  switch (type) {{
    case 'readdir': {{
      const segs = path.split('/').filter(Boolean);
      let dir = dirHandle;
      for (const s of segs) dir = await dir.getDirectoryHandle(s);
      const entries = [];
      for await (const entry of dir.values()) {{
        let size = 0, modified = new Date().toISOString();
        if (entry.kind === 'file') {{
          const f = await entry.getFile(); size = f.size;
          modified = new Date(f.lastModified).toISOString();
        }}
        entries.push({{ name: entry.name, is_dir: entry.kind === 'directory', size, modified }});
      }}
      entries.sort((a,b) => a.is_dir !== b.is_dir ? (a.is_dir ? -1 : 1) : a.name.localeCompare(b.name));
      return entries;
    }}
    case 'readFile': {{
      const {{ parent, name }} = await resolvePath(path);
      const fh = await parent.getFileHandle(name);
      const file = await fh.getFile();
      const buf = await file.arrayBuffer();
      const bytes = new Uint8Array(buf);
      let bin = '';
      for (let i = 0; i < bytes.length; i++) bin += String.fromCharCode(bytes[i]);
      return {{ data: btoa(bin), size: file.size, name: file.name, type: file.type, modified: new Date(file.lastModified).toISOString() }};
    }}
    case 'writeFile': {{
      const {{ parent, name }} = await resolvePath(path);
      const fh = await parent.getFileHandle(name, {{ create: true }});
      const w = await fh.createWritable();
      const bin = atob(cmd.data);
      const bytes = new Uint8Array(bin.length);
      for (let i = 0; i < bin.length; i++) bytes[i] = bin.charCodeAt(i);
      await w.write(bytes); await w.close();
      return {{ written: bytes.length }};
    }}
    case 'mkdir': {{
      const {{ parent, name }} = await resolvePath(path);
      await parent.getDirectoryHandle(name, {{ create: true }});
      return {{ created: true }};
    }}
    case 'delete': {{
      const {{ parent, name }} = await resolvePath(path);
      await parent.removeEntry(name, {{ recursive: true }});
      return {{ deleted: true }};
    }}
    case 'rename': {{
      const {{ parent: op, name: on }} = await resolvePath(cmd.oldPath);
      const {{ parent: np, name: nn }} = await resolvePath(cmd.newPath);
      try {{
        const fh = await op.getFileHandle(on);
        const f = await fh.getFile();
        const data = await f.arrayBuffer();
        const nfh = await np.getFileHandle(nn, {{ create: true }});
        const w = await nfh.createWritable();
        await w.write(data); await w.close();
        await op.removeEntry(on);
        return {{ renamed: true }};
      }} catch {{ throw new Error('ディレクトリのリネームはFile System Access APIでは非対応です'); }}
    }}
    case 'stat': {{
      const segs = path.split('/').filter(Boolean);
      if (segs.length === 0) return {{ name: folderName, is_dir: true, size: 0, modified: new Date().toISOString() }};
      const {{ parent, name }} = await resolvePath(path);
      try {{
        const fh = await parent.getFileHandle(name);
        const f = await fh.getFile();
        return {{ name: f.name, is_dir: false, size: f.size, modified: new Date(f.lastModified).toISOString() }};
      }} catch {{
        return {{ name, is_dir: true, size: 0, modified: new Date().toISOString() }};
      }}
    }}
    default: throw new Error('不明なコマンド: ' + type);
  }}
}}

async function handleConnect() {{
  showError(null);
  if (!('showDirectoryPicker' in window)) {{
    showError('File System Access APIに対応していません。このHTMLファイルをChromeまたはEdgeで直接開いてください（file://プロトコル）。');
    return;
  }}
  const serverIp = document.getElementById('server-ip').value.trim();
  if (!serverIp) {{
    showError('MacサーバーのIPアドレスを入力してください。');
    return;
  }}
  try {{
    dirHandle = await window.showDirectoryPicker({{ mode: 'readwrite' }});
    folderName = dirHandle.name;
    document.getElementById('folder-name').textContent = folderName;
    setStatus('connecting');
    log('フォルダ選択: ' + folderName);
    connectWS(serverIp);
  }} catch (err) {{
    if (err.name === 'AbortError') return;
    showError('フォルダの選択に失敗しました: ' + err.message);
  }}
}}

function connectWS(serverIp) {{
  const wsUrl = 'ws://' + serverIp + ':{port}/ws';
  log(wsUrl + ' に接続中...');
  ws = new WebSocket(wsUrl);
  ws.onopen = () => {{
    log('WebSocket接続完了、登録中...');
    const pcName = document.getElementById('pc-name').value.trim()
      || (navigator.userAgent.includes('Windows') ? 'Windows PC' : 'Client PC');
    ws.send(JSON.stringify({{
      type: 'register',
      name: pcName,
      folderName: folderName,
      token: '{api_token}',
    }}));
  }};
  ws.onmessage = async (event) => {{
    try {{
      const msg = JSON.parse(event.data);
      if (msg.type === 'registered') {{
        clientId = msg.clientId;
        document.getElementById('client-id').textContent = clientId;
        const pcNameVal = document.getElementById('pc-name').value.trim()
          || (navigator.userAgent.includes('Windows') ? 'Windows PC' : 'Client PC');
        document.getElementById('connected-name').textContent = pcNameVal;
        const serverIpVal = document.getElementById('server-ip').value.trim();
        document.getElementById('webdav-url').textContent =
          'http://' + serverIpVal + ':{port}/webdav/' + clientId + '/';
        setStatus('connected');
        log('クライアント登録完了: ' + clientId);
        log('WebDAV URL: http://' + serverIpVal + ':{port}/webdav/' + clientId + '/');
        return;
      }}
      const id = msg.id;
      try {{
        const result = await handleCommand(msg);
        ws.send(JSON.stringify({{ id, ok: true, data: result }}));
        log('OK: ' + msg.type + ' ' + (msg.path || ''));
      }} catch (err) {{
        ws.send(JSON.stringify({{ id, ok: false, error: err.message }}));
        log('ERR: ' + msg.type + ' - ' + err.message);
      }}
    }} catch {{ log('メッセージの解析に失敗'); }}
  }};
  ws.onclose = () => {{
    log('切断されました');
    setStatus('disconnected');
    clientId = '';
    ws = null;
  }};
  ws.onerror = () => {{
    log('WebSocketエラー');
    showError('接続に失敗しました。Rustサーバー(ポート{port})が起動しているか確認してください。IP: ' + serverIp);
  }};
}}

function handleDisconnect() {{
  if (ws) ws.close();
  dirHandle = null;
  setStatus('disconnected');
  folderName = '';
  clientId = '';
}}
</script>
</body>
</html>"##, server_ip = server_ip, port = port, api_token = api_token)
}
