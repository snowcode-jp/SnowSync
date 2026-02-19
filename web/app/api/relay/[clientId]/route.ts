// SNOWCODE - ソフトウェア製品
// (C) SNOWCODE
// 開発者: 雪符しき
// https://snowcode.jp
// 問い合わせ: info@snowcode.jp

import { NextRequest, NextResponse } from "next/server";

const RUST_SERVER = process.env.RUST_SERVER_URL ?? "http://localhost:17200";

export async function POST(
  request: NextRequest,
  { params }: { params: Promise<{ clientId: string }> }
) {
  const { clientId } = await params;
  const body = await request.json();

  try {
    const res = await fetch(`${RUST_SERVER}/api/relay/${clientId}`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(body),
    });

    const data = await res.json();
    return NextResponse.json(data, { status: res.status });
  } catch {
    return NextResponse.json(
      { error: "Failed to connect to relay server" },
      { status: 502 }
    );
  }
}
