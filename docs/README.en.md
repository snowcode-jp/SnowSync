# SnowSync

**[日本語](../README.md)** | **[English](#)** | **[中文](README.zh.md)**

### Access your Windows files from Mac — instantly, over your local network.

No cloud. No USB drives. No third-party accounts. Just open a folder on your Windows PC and it appears in your Mac's Finder.

| Login | Dashboard |
|:---:|:---:|
| ![Login](assets/01_auth.png) | ![Dashboard](assets/02_dashboard.png) |

| Connect | File Browser |
|:---:|:---:|
| ![Connect](assets/03_connect.png) | ![File Browser](assets/04_browse.png) |

---

## The Problem

You use both Mac and Windows. You need a file from your Windows PC, but:
- Cloud sync is slow and eats storage
- USB drives require walking back and forth
- Network shares (SMB) are unreliable between Mac and Windows
- Existing tools need complex setup or paid licenses

## The Solution

SnowSync turns any Windows folder into a native Mac drive — in 3 clicks.

1. Run SnowSync on your Mac
2. Open a webpage on your Windows PC
3. Pick a folder to share

That's it. The folder appears in Finder. Browse, edit, copy — everything works like a local drive.

---

## Features

**For everyday use:**
- Mount Windows folders directly in macOS Finder
- Browse, upload, download, rename, and delete files from your browser
- Connect multiple Windows PCs at the same time
- Works entirely on your local network — nothing leaves your LAN

**Under the hood:**
- Real-time WebSocket relay between Mac and Windows
- WebDAV mounting with automatic HTTP/HTTPS fallback
- Self-signed TLS certificates generated automatically
- Token-based API authentication
- Built with Rust (server) and Next.js (dashboard)

---

## Quick Start

### Requirements
- **Mac**: macOS 13+, [Rust](https://rustup.rs/), [Node.js](https://nodejs.org/) 20+
- **Windows**: Chrome 86+ or Edge 86+

### Setup

```bash
# Clone and install
git clone https://github.com/snowcode-jp/SnowSync.git
cd SnowSync && cd web && npm install && cd ..

# Start SnowSync
bash scripts/dev.sh
```

On startup, an **API Token** is displayed in the terminal:

```
  API Token: xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx
```

Copy this token — you'll need it to log into the dashboard.

### Log into the Dashboard

1. Open `http://localhost:17100` in your browser
2. Paste the API Token shown in the terminal
3. Click **Connect**

### Connect a Windows PC

1. On your Windows PC, open Chrome/Edge and go to `http://<your-mac-ip>:17200`
2. Download and open the connection file
3. Select a folder to share — done!

Your files are now accessible from your Mac's Finder and the web dashboard.

---

## How It Works

```
  Windows PC                          Mac
  ┌──────────────┐          ┌─────────────────────┐
  │ Select folder │  ◄─────► │  SnowSync Server    │
  │ in browser    │ WebSocket│  ┌─ Finder (WebDAV) │
  └──────────────┘          │  └─ Web Dashboard    │
                            └─────────────────────┘
              Your local network — nothing goes online
```

## Security

- API authentication with auto-generated tokens
- Mount paths restricted to `~/Public/mount`
- Command whitelist prevents unauthorized operations
- CORS locked to localhost
- TLS encryption for WebDAV connections

---

## License

[MIT License](../LICENSE) - (C) 2026 SNOWCODE / Yukifu Shiki

Free to use, modify, and redistribute.

## Disclaimer

This software is provided "as is" without warranty. Use at your own risk.
