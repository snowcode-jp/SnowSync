# SnowSync

**[日本語](#概要)** | **[English](docs/README.en.md)** | **[中文](docs/README.zh.md)**

---

MacからWindowsの複数PCのストレージをマウント・操作できるリモートファイル共有システム

## 概要

SnowSyncは、LAN内のWindows PCの共有フォルダをMacからWebDAV経由でマウントし、Finderやブラウザから操作できるシステムです。

- WebSocket中継によるリアルタイム双方向通信
- macOS FinderへのWebDAVマウント（loopback HTTP / HTTPS自動切替）
- Webベースのファイルブラウザ（アップロード・ダウンロード・リネーム・削除）
- 複数Windows PCの同時接続対応
- 自己署名TLS証明書の自動生成
- トースト通知システム（成功・エラー・警告・情報）

## 技術スタック

| カテゴリ | 技術 | バージョン |
|----------|------|-----------|
| サーバー言語 | Rust | Edition 2021 |
| Webフレームワーク | Axum | 0.8 |
| 非同期ランタイム | Tokio | 1.x |
| WebDAVサーバー | dav-server | 0.8 |
| TLS | rustls + rcgen | 0.23 / 0.13 |
| HTTP/2 | hyper + hyper-util | 1.x / 0.1 |
| フロントエンド | Next.js (App Router) | 15.3+ |
| UI | React | 19.0+ |
| 型システム | TypeScript | 5.7+ |
| CSS | Tailwind CSS + カスタムCSS | 4.x |
| アイコン | Font Awesome (react) | 6.x |
| フォント | Zen Maru Gothic | - |
| ランタイム | Node.js | 20.x |

## アーキテクチャ

```
  Windows PC (Chrome/Edge)              Mac (サーバー)
  ┌─────────────────────┐     ┌────────────────────────────────┐
  │  ljc-connect.html   │     │  Rust Relay Server (:17200)    │
  │  (File System       │◄───►│  ├─ WebSocket /ws              │
  │   Access API)       │ WS  │  ├─ REST API /api/*            │
  │                     │     │  ├─ WebDAV /webdav/<id>/       │
  └─────────────────────┘     │  └─ HTTPS (:17201)             │
                              │                                │
                              │  Next.js Web UI (:17100)       │
                              │  ├─ ダッシュボード /            │
                              │  ├─ ファイル閲覧 /browse       │
                              │  └─ 接続ガイド /connect        │
                              │                                │
                              │  macOS Finder                  │
                              │  └─ WebDAV mount               │
                              └────────────────────────────────┘
```

## ソースツリー

```
SnowSync/
├── .env.example                  # 環境変数テンプレート
├── .gitignore
├── README.md
├── docs/
│   ├── README.en.md              # English documentation
│   └── README.zh.md              # 中文文档
├── scripts/
│   └── dev.sh                    # 開発用起動スクリプト（Rust + Next.js同時起動）
├── server/                       # Rust リレーサーバー
│   ├── Cargo.toml
│   ├── Cargo.lock
│   └── src/
│       ├── main.rs               # エントリポイント: HTTP + HTTPS デュアルサーバー
│       ├── config.rs             # 設定管理（環境変数 LJC_PORT, LJC_BIND）
│       ├── server.rs             # Axumルーター構築（API / WebSocket / WebDAV）
│       ├── state.rs              # アプリ状態: クライアント接続管理
│       ├── ws.rs                 # WebSocketハンドラ: クライアント登録・コマンド中継
│       ├── relay.rs              # REST API: クライアント一覧・コマンド転送
│       ├── webdav_bridge.rs      # WebDAV←→WebSocketブリッジ（RelayFs仮想FS）
│       ├── mount.rs              # WebDAVマウント/アンマウント（3段階フォールバック）
│       ├── tls.rs                # 自己署名TLS証明書の生成・キャッシュ
│       └── connect_html.rs       # Windows用接続HTMLの動的生成
└── web/                          # Next.js フロントエンド
    ├── package.json
    ├── tsconfig.json
    ├── next.config.ts
    ├── postcss.config.mjs
    ├── app/
    │   ├── layout.tsx            # ルートレイアウト（サイドバー + Providers）
    │   ├── globals.css           # 雪結晶テーマCSS
    │   ├── page.tsx              # ダッシュボード: 統計・クライアント一覧
    │   ├── browse/page.tsx       # ファイル閲覧: リモートファイルブラウザ
    │   ├── connect/page.tsx      # 接続ガイド: 接続HTMLダウンロード
    │   └── api/                  # Next.js APIルート（Rustサーバーへのプロキシ）
    │       ├── clients/route.ts
    │       ├── mount/route.ts
    │       ├── unmount/route.ts
    │       ├── mounts/route.ts
    │       ├── connect-html/route.ts
    │       └── relay/[clientId]/route.ts
    ├── components/
    │   ├── Sidebar.tsx           # サイドバーナビゲーション
    │   ├── ServerStatus.tsx      # サーバー情報統計カード
    │   ├── ClientList.tsx        # 接続クライアント一覧テーブル
    │   ├── RemoteBrowser.tsx     # リモートファイルブラウザ
    │   ├── MountInstructions.tsx # マウント手順ガイド
    │   ├── Toast.tsx             # トースト通知（Context + Hook）
    │   └── Providers.tsx         # クライアントコンポーネントラッパー
    └── lib/
        └── types.ts              # TypeScript型定義
```

## 必要な環境

- **Mac（サーバー側）**: macOS 13+, Rust 1.70+ (cargo), Node.js 20+ (npm/npx)
- **Windows（クライアント側）**: Chrome 86+ または Edge 86+（File System Access API対応）

## 起動手順

### 1. 依存関係のインストール

```bash
cd web && npm install && cd ..
```

### 2. 開発サーバーの起動

```bash
bash scripts/dev.sh
```

以下のサーバーが起動します:

| サービス | URL |
|----------|-----|
| Web UI | http://localhost:17100 |
| WebSocket中継 | ws://localhost:17200/ws |
| WebDAV (HTTP) | http://localhost:17200/webdav/\<client_id\>/ |
| WebDAV (HTTPS) | https://localhost:17201/webdav/\<client_id\>/ |

### 3. Windows PCの接続

1. Windows側のChromeまたはEdgeで `http://<Mac IP>:17200/` にアクセス
2. 表示されたページから **SnowSync-Connect.html** をダウンロード
3. ダウンロードしたHTMLファイルをChromeまたはEdgeで開く（`file://` プロトコル）
4. IPアドレスが自動入力されていることを確認し「❄ フォルダを選択して接続」をクリック
5. 共有したいフォルダを選択 → Mac側のダッシュボードにクライアントが表示される
6. Mac側のWeb UI (`http://localhost:17100`) またはFinderからファイルを操作

> **なぜHTMLダウンロードが必要？**
> ファイル操作に使用するFile System Access APIは、セキュリティ上 `file://` または `https://` でのみ動作します。
> `http://` で直接開くとフォルダ選択ができないため、ダウンロードして `file://` で開く必要があります。

> **セキュリティ**: ダッシュボード (ポート17100) はMacのローカルホストのみにバインドされており、クライアントPCからはアクセスできません。

## 環境変数

`.env.example` を `.env` にコピーして設定できます。未設定の場合はデフォルト値が使用されます。

| 変数 | デフォルト | 説明 |
|------|-----------|------|
| `WEB_PORT` | 17100 | Next.js Web UIのポート |
| `LJC_PORT` | 17200 | Rustリレーサーバーのポート（HTTPS = +1） |
| `LJC_BIND` | 0.0.0.0 | バインドアドレス |
| `RUST_SERVER_URL` | http://localhost:17200 | Next.jsからRustサーバーへの接続先 |

## ライセンス

[MIT License](LICENSE) - (C) 2026 SNOWCODE / 雪符しき

自由に使用・改変・再配布できます。詳細は [LICENSE](LICENSE) をご覧ください。

## 免責事項

本ソフトウェアは「現状有姿」で提供され、いかなる保証もありません。
使用は自己責任でお願いします。開発者は一切の責任を負いません。
