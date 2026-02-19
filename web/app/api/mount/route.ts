// SNOWCODE - ソフトウェア製品
// (C) SNOWCODE
// 開発者: 雪符しき
// https://snowcode.jp
// 問い合わせ: info@snowcode.jp

import { NextRequest, NextResponse } from "next/server";

const RUST_SERVER = process.env.RUST_SERVER_URL ?? "http://localhost:17200";

export async function POST(req: NextRequest) {
  const body = await req.json();
  try {
    const res = await fetch(`${RUST_SERVER}/api/mount`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(body),
    });
    const data = await res.json();
    return NextResponse.json(data, { status: res.status });
  } catch {
    return NextResponse.json(
      { error: "Failed to connect to server" },
      { status: 502 }
    );
  }
}
