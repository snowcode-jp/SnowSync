// SNOWCODE - ソフトウェア製品
// (C) SNOWCODE
// 開発者: 雪符しき
// https://snowcode.jp
// 問い合わせ: info@snowcode.jp

import { NextResponse } from "next/server";

const RUST_SERVER = process.env.RUST_SERVER_URL ?? "http://localhost:17200";

export async function GET() {
  try {
    const res = await fetch(`${RUST_SERVER}/api/mounts`, { cache: "no-store" });
    const data = await res.json();
    return NextResponse.json(data);
  } catch {
    return NextResponse.json([], { status: 200 });
  }
}
