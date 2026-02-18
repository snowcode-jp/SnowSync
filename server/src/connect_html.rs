// SNOWCODE - ソフトウェア製品
// (C) SNOWCODE
// 開発者: 雪符しき
// https://snowcode.jp
// 問い合わせ: info@snowcode.jp
// 本ソフトウェアは利用権の販売であり、著作権はSNOWCODEに帰属します。
// 署名の削除・改変は禁止されています。

use axum::extract::Query;
use axum::response::Html;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ConnectParams {
    pub ip: Option<String>,
    pub port: Option<u16>,
}

/// GET /api/connect-html?ip=192.168.x.x&port=17200
/// Returns a standalone HTML page with the server IP pre-filled.
pub async fn connect_html(Query(params): Query<ConnectParams>) -> Html<String> {
    let ip = params.ip.unwrap_or_default();
    let port = params.port.unwrap_or(17200);
    Html(generate_connect_html(&ip, port))
}

fn generate_connect_html(server_ip: &str, port: u16) -> String {
    format!(r##"<!DOCTYPE html>
<html lang="ja">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>LocalJackControl - 接続</title>
<style>
  * {{ margin: 0; padding: 0; box-sizing: border-box; }}
  body {{
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", system-ui, sans-serif;
    background: #0a0a0f; color: #e5e5e5; min-height: 100vh;
    display: flex; flex-direction: column; align-items: center;
    padding: 40px 20px;
  }}
  .container {{ max-width: 640px; width: 100%; }}
  h1 {{ font-size: 24px; margin-bottom: 8px; }}
  .subtitle {{ color: #888; font-size: 14px; margin-bottom: 32px; }}
  .card {{
    background: #111118; border: 1px solid #222; border-radius: 12px;
    padding: 24px; margin-bottom: 16px;
  }}
  .status-row {{ display: flex; align-items: center; gap: 12px; margin-bottom: 16px; }}
  .dot {{
    width: 12px; height: 12px; border-radius: 50%;
    transition: background 0.3s;
  }}
  .dot-off {{ background: #444; }}
  .dot-connecting {{ background: #eab308; animation: pulse 1.5s infinite; }}
  .dot-on {{ background: #22c55e; }}
  @keyframes pulse {{ 0%,100%{{opacity:1}} 50%{{opacity:0.4}} }}
  .status-text {{ font-size: 14px; font-weight: 500; }}
  .btn {{
    padding: 12px 24px; border: none; border-radius: 8px; cursor: pointer;
    font-size: 14px; font-weight: 500; transition: background 0.2s;
  }}
  .btn-primary {{ background: #2563eb; color: #fff; }}
  .btn-primary:hover {{ background: #3b82f6; }}
  .btn-primary:disabled {{ opacity: 0.5; cursor: not-allowed; }}
  .btn-danger {{ background: rgba(239,68,68,0.15); color: #f87171; }}
  .btn-danger:hover {{ background: rgba(239,68,68,0.25); }}
  .info-grid {{ display: grid; grid-template-columns: 1fr 1fr; gap: 16px; margin-bottom: 12px; }}
  .info-label {{ font-size: 11px; color: #666; text-transform: uppercase; letter-spacing: 0.5px; }}
  .info-value {{ font-family: monospace; font-size: 13px; margin-top: 4px; }}
  .log-box {{
    font-family: monospace; font-size: 11px; color: #666;
    max-height: 200px; overflow-y: auto; padding: 4px 0;
  }}
  .log-box p {{ margin: 2px 0; }}
  .log-title {{ font-size: 14px; color: #888; margin-bottom: 8px; }}
  .error-box {{
    background: rgba(239,68,68,0.1); border: 1px solid rgba(239,68,68,0.3);
    border-radius: 12px; padding: 16px; color: #f87171; margin-bottom: 16px;
    font-size: 14px;
  }}
  .server-input {{
    width: 100%; padding: 8px 12px; background: #1a1a24; border: 1px solid #333;
    border-radius: 8px; color: #e5e5e5; font-family: monospace; font-size: 14px;
    margin-bottom: 12px;
  }}
  .server-input:focus {{ outline: none; border-color: #2563eb; }}
  .input-label {{ font-size: 12px; color: #888; margin-bottom: 6px; }}
  .hidden {{ display: none; }}
</style>
</head>
<body>
<div class="container">
  <h1>LocalJackControl に接続</h1>
  <p class="subtitle">このPCのフォルダをMacサーバーに共有します。</p>

  <div id="error-box" class="error-box hidden"></div>

  <div class="card">
    <div class="input-label">MacサーバーのIPアドレス</div>
    <input id="server-ip" class="server-input" type="text" value="{server_ip}" placeholder="192.168.x.x" />

    <div class="input-label">このPCの名前（複数PC接続時に区別するため）</div>
    <input id="pc-name" class="server-input" type="text" value="" placeholder="例: デスクトップPC、ノートPC" />

    <div class="status-row">
      <div id="status-dot" class="dot dot-off"></div>
      <span id="status-text" class="status-text">未接続</span>
    </div>

    <div id="connect-section">
      <button id="btn-connect" class="btn btn-primary" onclick="handleConnect()">
        フォルダを選択して接続
      </button>
    </div>

    <div id="connected-section" class="hidden">
      <div class="info-grid">
        <div>
          <div class="info-label">PC名</div>
          <div id="connected-name" class="info-value">-</div>
        </div>
        <div>
          <div class="info-label">フォルダ</div>
          <div id="folder-name" class="info-value">-</div>
        </div>
      </div>
      <div class="info-grid" style="margin-top:8px;">
        <div>
          <div class="info-label">クライアントID</div>
          <div id="client-id" class="info-value" style="font-size:11px;color:#888;">-</div>
        </div>
        <div>
          <div class="info-label">WebDAV URL</div>
          <div id="webdav-url" class="info-value" style="font-size:11px;color:#888;word-break:break-all;">-</div>
        </div>
      </div>
      <button class="btn btn-danger" onclick="handleDisconnect()">切断</button>
    </div>
  </div>

  <div class="card">
    <div class="log-title">ログ</div>
    <div id="log-box" class="log-box">
      <p>接続待機中...</p>
    </div>
  </div>
</div>

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
  const connectSec = document.getElementById('connect-section');
  const connectedSec = document.getElementById('connected-section');
  const cls = {{ disconnected:'dot-off', connecting:'dot-connecting', connected:'dot-on' }};
  const labels = {{ disconnected:'未接続', connecting:'接続処理中...', connected:'接続中' }};
  dot.className = 'dot ' + (cls[s] || 'dot-off');
  text.textContent = labels[s] || '';
  if (s === 'connected') {{
    connectSec.classList.add('hidden');
    connectedSec.classList.remove('hidden');
  }} else {{
    connectSec.classList.remove('hidden');
    connectedSec.classList.add('hidden');
  }}
}}

function showError(msg) {{
  const box = document.getElementById('error-box');
  if (msg) {{ box.textContent = msg; box.classList.remove('hidden'); }}
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
</html>"##, server_ip = server_ip, port = port)
}
