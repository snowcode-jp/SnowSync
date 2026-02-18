# SnowSync

**[日本語](../README.md)** | **[English](README.en.md)** | **[中文](#概述)**

---

从Mac挂载和操作多台Windows PC存储的远程文件共享系统

## 概述

SnowSync是一个通过WebDAV将局域网内Windows PC的共享文件夹挂载到Mac上，并可通过macOS Finder或Web浏览器进行文件操作的系统。

- 通过WebSocket中继实现实时双向通信
- 在macOS Finder中挂载WebDAV（loopback HTTP / HTTPS自动切换）
- 基于Web的文件浏览器（上传、下载、重命名、删除）
- 支持多台Windows PC同时连接
- 自动生成自签名TLS证书
- Toast通知系统（成功、错误、警告、信息）

## 技术栈

| 类别 | 技术 | 版本 |
|------|------|------|
| 服务端语言 | Rust | Edition 2021 |
| Web框架 | Axum | 0.8 |
| 异步运行时 | Tokio | 1.x |
| WebDAV服务器 | dav-server | 0.8 |
| TLS | rustls + rcgen | 0.23 / 0.13 |
| HTTP/2 | hyper + hyper-util | 1.x / 0.1 |
| 前端框架 | Next.js (App Router) | 15.3+ |
| UI库 | React | 19.0+ |
| 类型系统 | TypeScript | 5.7+ |
| CSS | Tailwind CSS + 自定义CSS | 4.x |
| 图标 | Font Awesome (react) | 6.x |
| 字体 | Zen Maru Gothic | - |
| 运行时 | Node.js | 20.x |

## 架构

```
  Windows PC (Chrome/Edge)              Mac（服务端）
  ┌─────────────────────┐     ┌────────────────────────────────┐
  │  ljc-connect.html   │     │  Rust中继服务器 (:17200)        │
  │  (File System       │◄───►│  ├─ WebSocket /ws              │
  │   Access API)       │ WS  │  ├─ REST API /api/*            │
  │                     │     │  ├─ WebDAV /webdav/<id>/       │
  └─────────────────────┘     │  └─ HTTPS (:17201)             │
                              │                                │
                              │  Next.js Web UI (:17100)       │
                              │  ├─ 仪表盘 /                    │
                              │  ├─ 文件浏览 /browse            │
                              │  └─ 连接指南 /connect           │
                              │                                │
                              │  macOS Finder                  │
                              │  └─ WebDAV挂载                  │
                              └────────────────────────────────┘
```

## 源码结构

```
SnowSync/
├── .env.example                  # 环境变量模板
├── .gitignore
├── README.md                     # 日本语（主文档）
├── docs/
│   ├── README.en.md              # English
│   └── README.zh.md              # 中文（本文件）
├── scripts/
│   └── dev.sh                    # 开发启动脚本（同时启动Rust + Next.js）
├── server/                       # Rust中继服务器
│   ├── Cargo.toml
│   ├── Cargo.lock
│   └── src/
│       ├── main.rs               # 入口：HTTP + HTTPS双服务器
│       ├── config.rs             # 配置管理（环境变量：LJC_PORT, LJC_BIND）
│       ├── server.rs             # Axum路由（API / WebSocket / WebDAV）
│       ├── state.rs              # 应用状态：客户端连接管理
│       ├── ws.rs                 # WebSocket：客户端注册与命令中继
│       ├── relay.rs              # REST API：客户端列表与命令转发
│       ├── webdav_bridge.rs      # WebDAV↔WebSocket桥接（RelayFs虚拟FS）
│       ├── mount.rs              # WebDAV挂载/卸载（3级回退策略）
│       ├── tls.rs                # 自签名TLS证书生成与缓存
│       └── connect_html.rs       # Windows客户端连接HTML动态生成
└── web/                          # Next.js前端
    ├── package.json
    ├── tsconfig.json
    ├── next.config.ts
    ├── postcss.config.mjs
    ├── app/
    │   ├── layout.tsx            # 根布局（侧边栏 + Providers）
    │   ├── globals.css           # 雪花结晶主题CSS
    │   ├── page.tsx              # 仪表盘：统计、客户端列表
    │   ├── browse/page.tsx       # 文件浏览：远程文件操作
    │   ├── connect/page.tsx      # 连接指南：HTML下载
    │   └── api/                  # Next.js API路由（代理到Rust）
    │       ├── clients/route.ts
    │       ├── mount/route.ts
    │       ├── unmount/route.ts
    │       ├── mounts/route.ts
    │       ├── connect-html/route.ts
    │       └── relay/[clientId]/route.ts
    ├── components/
    │   ├── Sidebar.tsx           # 侧边栏导航
    │   ├── ServerStatus.tsx      # 服务器信息统计卡片
    │   ├── ClientList.tsx        # 已连接客户端列表
    │   ├── RemoteBrowser.tsx     # 远程文件浏览器
    │   ├── MountInstructions.tsx # 挂载说明指南
    │   ├── Toast.tsx             # Toast通知（Context + Hook）
    │   └── Providers.tsx         # 客户端组件包装器
    └── lib/
        └── types.ts              # TypeScript类型定义
```

## 系统要求

- **Mac（服务端）**: macOS 13+, Rust 1.70+ (cargo), Node.js 20+ (npm/npx)
- **Windows（客户端）**: Chrome 86+ 或 Edge 86+（支持File System Access API）

## 启动步骤

### 1. 安装依赖

```bash
cd web && npm install && cd ..
```

### 2. 启动开发服务器

```bash
bash scripts/dev.sh
```

以下服务将启动：

| 服务 | URL |
|------|-----|
| Web UI | http://localhost:17100 |
| WebSocket中继 | ws://localhost:17200/ws |
| WebDAV (HTTP) | http://localhost:17200/webdav/\<client_id\>/ |
| WebDAV (HTTPS) | https://localhost:17201/webdav/\<client_id\>/ |

### 3. 连接Windows PC

1. 从Mac的Web UI `http://<Mac IP>:17100/connect` 下载连接用HTML
2. 将HTML文件传输到Windows PC，用Chrome/Edge打开
3. 点击"选择文件夹并连接"，选择要共享的文件夹
4. 从Mac端的Web UI或Finder操作文件

## 环境变量

将 `.env.example` 复制为 `.env` 进行自定义配置。未设置时使用默认值。

| 变量 | 默认值 | 说明 |
|------|--------|------|
| `WEB_PORT` | 17100 | Next.js Web UI端口 |
| `LJC_PORT` | 17200 | Rust中继服务器端口（HTTPS = +1） |
| `LJC_BIND` | 0.0.0.0 | 绑定地址 |
| `RUST_SERVER_URL` | http://localhost:17200 | Next.js连接Rust服务器的地址 |

## 许可证

```
SNOWCODE - 软件产品
(C) SNOWCODE
开发者: 雪符しき
https://snowcode.jp
联系方式: info@snowcode.jp
```

本软件以使用许可形式销售，著作权归SNOWCODE所有。
禁止删除或修改版权声明。

## 免责声明

本软件按"原样"提供，不附带任何明示或暗示的保证。
开发者对因使用本软件而产生的任何损害概不负责。
用户应自行承担使用本软件的全部责任。
