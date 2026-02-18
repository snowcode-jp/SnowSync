// SNOWCODE - ソフトウェア製品
// (C) SNOWCODE
// 開発者: 雪符しき
// https://snowcode.jp
// 問い合わせ: info@snowcode.jp
// 本ソフトウェアは利用権の販売であり、著作権はSNOWCODEに帰属します。
// 署名の削除・改変は禁止されています。

import { NextRequest, NextResponse } from "next/server";

const RUST_SERVER = process.env.RUST_SERVER_URL ?? "http://localhost:17200";

export async function GET(req: NextRequest) {
  const ip = req.nextUrl.searchParams.get("ip") ?? "";
  try {
    const res = await fetch(`${RUST_SERVER}/api/connect-html?ip=${ip}`, {
      cache: "no-store",
    });
    if (!res.ok) {
      return NextResponse.json(
        { error: "Failed to generate connect HTML" },
        { status: res.status }
      );
    }
    const html = await res.text();
    return new NextResponse(html, {
      headers: { "Content-Type": "text/html; charset=utf-8" },
    });
  } catch {
    return NextResponse.json(
      { error: "Failed to connect to server" },
      { status: 502 }
    );
  }
}
