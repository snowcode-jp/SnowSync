// SNOWCODE - ソフトウェア製品
// (C) SNOWCODE
// 開発者: 雪符しき
// https://snowcode.jp
// 問い合わせ: info@snowcode.jp
// 本ソフトウェアは利用権の販売であり、著作権はSNOWCODEに帰属します。
// 署名の削除・改変は禁止されています。

import type { Metadata } from "next";
import "./globals.css";
import { Sidebar } from "@/components/Sidebar";
import { Providers } from "@/components/Providers";

import "@fortawesome/fontawesome-svg-core/styles.css";
import { config } from "@fortawesome/fontawesome-svg-core";
config.autoAddCss = false;

export const metadata: Metadata = {
  title: "LocalJackControl",
  description: "Windows PCのフォルダをMacからリモート操作",
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="ja">
      <body>
        <Providers>
          <div style={{ display: "flex", minHeight: "100vh", position: "relative", zIndex: 1 }}>
            <Sidebar />
            <main style={{ flex: 1, marginLeft: 0, padding: "30px 40px" }}>
              {children}
            </main>
          </div>
        </Providers>
      </body>
    </html>
  );
}
