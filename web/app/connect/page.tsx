// SNOWCODE - ソフトウェア製品
// (C) SNOWCODE
// 開発者: 雪符しき
// https://snowcode.jp
// 問い合わせ: info@snowcode.jp
// 本ソフトウェアは利用権の販売であり、著作権はSNOWCODEに帰属します。
// 署名の削除・改変は禁止されています。

"use client";

import { useState, useEffect } from "react";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import {
  faSnowflake,
  faPlug,
  faDownload,
  faListOl,
  faCircleQuestion,
  faTriangleExclamation,
} from "@fortawesome/free-solid-svg-icons";

export default function ConnectPage() {
  const [ip, setIp] = useState("...");

  useEffect(() => {
    setIp(window.location.hostname);
  }, []);

  const handleDownload = async () => {
    try {
      const res = await fetch(`/api/connect-html?ip=${ip}`);
      if (!res.ok) throw new Error("サーバーに接続できません");
      const html = await res.text();
      const blob = new Blob([html], { type: "text/html" });
      const a = document.createElement("a");
      a.href = URL.createObjectURL(blob);
      a.download = "ljc-connect.html";
      a.click();
      URL.revokeObjectURL(a.href);
    } catch {
      alert("サーバーに接続できません。Rustサーバーが起動しているか確認してください。");
    }
  };

  return (
    <div>
      {/* Page Header */}
      <div className="page-header">
        <h1 className="page-title">
          <FontAwesomeIcon icon={faSnowflake} style={{ color: "#7eb8d8", fontSize: 22 }} />
          <FontAwesomeIcon icon={faPlug} style={{ color: "#7eb8d8", fontSize: 22 }} />
          Windows PCから接続
        </h1>
      </div>

      {/* メインカード */}
      <div className="card">
        {/* ダウンロードセクション */}
        <div className="card-header">
          <div className="card-title">
            <FontAwesomeIcon icon={faDownload} />
            接続用HTMLをダウンロード
          </div>
        </div>
        <p style={{ fontSize: 14, color: "#4a6b7c", marginBottom: 20, lineHeight: 1.8 }}>
          以下のボタンで接続用HTMLをダウンロードし、Windows PCの
          <strong style={{ color: "#4a7c9b" }}> Chrome </strong>または
          <strong style={{ color: "#4a7c9b" }}> Edge </strong>で開いてください。
          MacサーバーのIPアドレス（
          <code style={{ color: "#5a9fc8", fontFamily: "monospace", fontWeight: 600 }}>{ip}</code>
          ）が自動で埋め込まれます。
        </p>
        <button onClick={handleDownload} className="btn">
          <FontAwesomeIcon icon={faDownload} />
          接続HTML をダウンロード
        </button>
      </div>

      {/* 使い方カード */}
      <div className="card">
        <div className="card-header">
          <div className="card-title">
            <FontAwesomeIcon icon={faListOl} />
            使い方
          </div>
        </div>
        <ol style={{ display: "flex", flexDirection: "column", gap: 12 }}>
          {[
            <>上のボタンで <code style={{ color: "#5a9fc8", fontFamily: "monospace" }}>ljc-connect.html</code> をダウンロード</>,
            <>そのHTMLファイルをWindows PCにコピー（USB・メール・共有フォルダなど）</>,
            <>Windows PCでChrome/Edgeを開き、HTMLファイルをダブルクリックで開く</>,
            <>「フォルダを選択して接続」をクリックし、共有したいフォルダを選択</>,
            <>ブラウザタブを<strong style={{ color: "#4a7c9b" }}>開いたまま</strong>にする（閉じると切断されます）</>,
          ].map((text, i) => (
            <li
              key={i}
              style={{
                fontSize: 14,
                color: "#4a6b7c",
                display: "flex",
                gap: 12,
                alignItems: "flex-start",
              }}
            >
              <span
                style={{
                  display: "inline-flex",
                  alignItems: "center",
                  justifyContent: "center",
                  width: 28,
                  height: 28,
                  borderRadius: 14,
                  background: "linear-gradient(135deg, #7eb8d8 0%, #5a9fc8 100%)",
                  color: "#fff",
                  fontSize: 13,
                  fontWeight: 700,
                  flexShrink: 0,
                }}
              >
                {i + 1}
              </span>
              <span style={{ paddingTop: 3 }}>{text}</span>
            </li>
          ))}
        </ol>
      </div>

      {/* なぜHTMLファイルが必要？ */}
      <div className="card">
        <div className="card-header">
          <div className="card-title">
            <FontAwesomeIcon icon={faCircleQuestion} />
            なぜHTMLファイルが必要？
          </div>
        </div>
        <p style={{ fontSize: 14, color: "#4a6b7c", lineHeight: 1.8 }}>
          File System Access APIはセキュリティ上、
          <code style={{ color: "#5a9fc8", fontFamily: "monospace", fontWeight: 600 }}>https://</code> または
          <code style={{ color: "#5a9fc8", fontFamily: "monospace", fontWeight: 600 }}> file://</code> でしか動作しません。
          ローカルネットワークの
          <code style={{ color: "#5a9fc8", fontFamily: "monospace", fontWeight: 600 }}> http://</code> では使えないため、
          HTMLファイルをダウンロードして
          <code style={{ color: "#5a9fc8", fontFamily: "monospace", fontWeight: 600 }}> file://</code> で開く方式を採用しています。
        </p>
      </div>

      {/* 注意 */}
      <div className="alert alert-warning">
        <FontAwesomeIcon icon={faTriangleExclamation} />
        <span>
          Windows 2台以上の接続も可能です。各PCでそれぞれHTMLを開いてフォルダを共有してください。
          Mac側のダッシュボードで全クライアントを管理できます。
        </span>
      </div>
    </div>
  );
}
