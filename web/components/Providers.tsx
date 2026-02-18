// SNOWCODE - ソフトウェア製品
// (C) SNOWCODE
// 開発者: 雪符しき
// https://snowcode.jp
// 問い合わせ: info@snowcode.jp
// 本ソフトウェアは利用権の販売であり、著作権はSNOWCODEに帰属します。
// 署名の削除・改変は禁止されています。

"use client";

import { ToastProvider } from "@/components/Toast";

export function Providers({ children }: { children: React.ReactNode }) {
  return <ToastProvider>{children}</ToastProvider>;
}
