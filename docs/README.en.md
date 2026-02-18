# SnowSync

**[日本語](../README.md)** | **[English](#overview)** | **[中文](README.zh.md)**

---

A remote file sharing system that lets you mount and manage storage from multiple Windows PCs on your Mac.

## Overview

SnowSync mounts shared folders from Windows PCs on your LAN via WebDAV, allowing file operations through macOS Finder or a web browser.

- Real-time bidirectional communication via WebSocket relay
- WebDAV mounting in macOS Finder (loopback HTTP / HTTPS auto-fallback)
- Web-based file browser (upload, download, rename, delete)
- Simultaneous multi-client Windows PC support
- Automatic self-signed TLS certificate generation
- Toast notification system (success, error, warning, info)

## Tech Stack

| Category | Technology | Version |
|----------|-----------|---------|
| Server Language | Rust | Edition 2021 |
| Web Framework | Axum | 0.8 |
| Async Runtime | Tokio | 1.x |
| WebDAV Server | dav-server | 0.8 |
| TLS | rustls + rcgen | 0.23 / 0.13 |
| HTTP/2 | hyper + hyper-util | 1.x / 0.1 |
| Frontend | Next.js (App Router) | 15.3+ |
| UI | React | 19.0+ |
| Type System | TypeScript | 5.7+ |
| CSS | Tailwind CSS + Custom CSS | 4.x |
| Icons | Font Awesome (react) | 6.x |
| Font | Zen Maru Gothic | - |
| Runtime | Node.js | 20.x |

## Architecture

```
  Windows PC (Chrome/Edge)              Mac (Server)
  ┌─────────────────────┐     ┌────────────────────────────────┐
  │  ljc-connect.html   │     │  Rust Relay Server (:17200)    │
  │  (File System       │◄───►│  ├─ WebSocket /ws              │
  │   Access API)       │ WS  │  ├─ REST API /api/*            │
  │                     │     │  ├─ WebDAV /webdav/<id>/       │
  └─────────────────────┘     │  └─ HTTPS (:17201)             │
                              │                                │
                              │  Next.js Web UI (:17100)       │
                              │  ├─ Dashboard /                │
                              │  ├─ File Browser /browse       │
                              │  └─ Connect Guide /connect     │
                              │                                │
                              │  macOS Finder                  │
                              │  └─ WebDAV mount               │
                              └────────────────────────────────┘
```

## Source Tree

```
SnowSync/
├── .env.example                  # Environment variable template
├── .gitignore
├── README.md                     # Japanese (main)
├── docs/
│   ├── README.en.md              # English (this file)
│   └── README.zh.md              # Chinese
├── scripts/
│   └── dev.sh                    # Dev launcher (starts Rust + Next.js)
├── server/                       # Rust relay server
│   ├── Cargo.toml
│   ├── Cargo.lock
│   └── src/
│       ├── main.rs               # Entry: dual HTTP + HTTPS server
│       ├── config.rs             # Config (env vars: LJC_PORT, LJC_BIND)
│       ├── server.rs             # Axum router (API / WebSocket / WebDAV)
│       ├── state.rs              # App state: client connection management
│       ├── ws.rs                 # WebSocket: client registration & relay
│       ├── relay.rs              # REST API: client listing & command forwarding
│       ├── webdav_bridge.rs      # WebDAV↔WebSocket bridge (RelayFs virtual FS)
│       ├── mount.rs              # WebDAV mount/unmount (3-stage fallback)
│       ├── tls.rs                # Self-signed TLS cert generation & caching
│       └── connect_html.rs       # Dynamic HTML generation for Windows clients
└── web/                          # Next.js frontend
    ├── package.json
    ├── tsconfig.json
    ├── next.config.ts
    ├── postcss.config.mjs
    ├── app/
    │   ├── layout.tsx            # Root layout (sidebar + providers)
    │   ├── globals.css           # Snow crystal theme CSS
    │   ├── page.tsx              # Dashboard: stats, clients
    │   ├── browse/page.tsx       # File browser: remote file operations
    │   ├── connect/page.tsx      # Connection guide: HTML download
    │   └── api/                  # Next.js API routes (proxy to Rust)
    │       ├── clients/route.ts
    │       ├── mount/route.ts
    │       ├── unmount/route.ts
    │       ├── mounts/route.ts
    │       ├── connect-html/route.ts
    │       └── relay/[clientId]/route.ts
    ├── components/
    │   ├── Sidebar.tsx           # Sidebar navigation
    │   ├── ServerStatus.tsx      # Server info stat cards
    │   ├── ClientList.tsx        # Connected clients table
    │   ├── RemoteBrowser.tsx     # Remote file browser
    │   ├── MountInstructions.tsx # Mount instructions guide
    │   ├── Toast.tsx             # Toast notifications (Context + Hook)
    │   └── Providers.tsx         # Client component wrapper
    └── lib/
        └── types.ts              # TypeScript type definitions
```

## Requirements

- **Mac (server)**: macOS 13+, Rust 1.70+ (cargo), Node.js 20+ (npm/npx)
- **Windows (client)**: Chrome 86+ or Edge 86+ (File System Access API support)

## Getting Started

### 1. Install dependencies

```bash
cd web && npm install && cd ..
```

### 2. Start the development servers

```bash
bash scripts/dev.sh
```

The following services will start:

| Service | URL |
|---------|-----|
| Web UI | http://localhost:17100 |
| WebSocket Relay | ws://localhost:17200/ws |
| WebDAV (HTTP) | http://localhost:17200/webdav/\<client_id\>/ |
| WebDAV (HTTPS) | https://localhost:17201/webdav/\<client_id\>/ |

### 3. Connect a Windows PC

1. Download the connection HTML from Mac's Web UI at `http://<Mac IP>:17100/connect`
2. Transfer the HTML file to the Windows PC and open it in Chrome/Edge
3. Click "Select folder and connect" to choose a folder to share
4. Manage files from the Mac via the Web UI or Finder

## Environment Variables

Copy `.env.example` to `.env` to customize. Default values are used when not set.

| Variable | Default | Description |
|----------|---------|-------------|
| `WEB_PORT` | 17100 | Next.js Web UI port |
| `LJC_PORT` | 17200 | Rust relay server port (HTTPS = +1) |
| `LJC_BIND` | 0.0.0.0 | Bind address |
| `RUST_SERVER_URL` | http://localhost:17200 | Next.js connection to Rust server |

## License

```
SNOWCODE - Software Product
(C) SNOWCODE
Developer: Yukifu Shiki
https://snowcode.jp
Contact: info@snowcode.jp
```

This software is sold as a license to use. All copyrights belong to SNOWCODE.
Removal or modification of the copyright notice is prohibited.

## Disclaimer

This software is provided "as is" without warranty of any kind, express or implied.
The developer shall not be liable for any damages arising from the use of this software.
Users assume all responsibility for the use of this software.
