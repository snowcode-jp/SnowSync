#!/bin/bash
# SNOWCODE - ソフトウェア製品
# (C) SNOWCODE
# 開発者: 雪符しき
# https://snowcode.jp
# 問い合わせ: info@snowcode.jp
# 本ソフトウェアは利用権の販売であり、著作権はSNOWCODEに帰属します。
# 署名の削除・改変は禁止されています。

set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

WEB_PORT="${WEB_PORT:-17100}"
LJC_PORT="${LJC_PORT:-17200}"
LJC_HTTPS_PORT=$((LJC_PORT + 1))

echo "=== LocalJackControl Development ==="
echo ""

# Start Rust relay server in background
echo "[1/2] Starting Rust relay server (HTTP $LJC_PORT + HTTPS $LJC_HTTPS_PORT)..."
(cd "$PROJECT_DIR/server" && LJC_PORT=$LJC_PORT cargo run) &
RUST_PID=$!

# Start Next.js dev server in background
echo "[2/2] Starting Next.js dev server (port $WEB_PORT)..."
(cd "$PROJECT_DIR/web" && npx next dev -p "$WEB_PORT" -H 0.0.0.0) &
NEXT_PID=$!

# Trap to kill both on exit
trap "kill $RUST_PID $NEXT_PID 2>/dev/null; exit" INT TERM EXIT

echo ""
echo "Both servers starting..."
echo "  Web UI:       http://localhost:$WEB_PORT"
echo "  Relay WS:     ws://localhost:$LJC_PORT/ws"
echo "  WebDAV HTTP:  http://localhost:$LJC_PORT/webdav/<client_id>/"
echo "  WebDAV HTTPS: https://localhost:$LJC_HTTPS_PORT/webdav/<client_id>/"
echo ""
echo "  Windows: Open http://<Mac IP>:$WEB_PORT/connect in Chrome/Edge"
echo "  Mac:     Open http://localhost:$WEB_PORT/browse"
echo "  Finder:  「Finderで開く」ボタン (HTTPS自動マウント)"
echo ""
echo "Press Ctrl+C to stop both servers."

wait
