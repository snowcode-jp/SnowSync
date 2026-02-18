// SNOWCODE - ソフトウェア製品
// (C) SNOWCODE
// 開発者: 雪符しき
// https://snowcode.jp
// 問い合わせ: info@snowcode.jp
// 本ソフトウェアは利用権の販売であり、著作権はSNOWCODEに帰属します。
// 署名の削除・改変は禁止されています。

"use client";

import { useEffect, useState } from "react";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import {
  faWindows,
} from "@fortawesome/free-brands-svg-icons";
import {
  faGlobe,
  faHardDrive,
  faTriangleExclamation,
  faListCheck,
} from "@fortawesome/free-solid-svg-icons";

export function ConnectionGuide() {
  const [ip, setIp] = useState("...");

  useEffect(() => {
    setIp(window.location.hostname);
  }, []);

  const steps = [
    {
      icon: faWindows,
      title: "Windows PC (フォルダを共有)",
      items: [
        "「接続」ページから接続用HTMLをダウンロード",
        "Windows PCにHTMLファイルをコピー",
        "Chrome / Edgeでそのファイルを開く",
        "「フォルダを選択して接続」をクリック",
        "ブラウザタブは開いたままにする",
      ],
    },
    {
      icon: faGlobe,
      title: "Mac (ブラウザでファイル操作)",
      items: [
        "「ファイル閲覧」またはクライアント一覧から「閲覧」",
        "接続中のクライアントを選択",
        "閲覧・ダウンロード・アップロード・削除・リネーム可能",
      ],
    },
    {
      icon: faHardDrive,
      title: "Mac (Finder / VSCode / Zed)",
      items: [
        "クライアント一覧の「Finderで開く」をクリック",
        "HTTPS (自己署名証明書) でFinderにマウントされます",
        "マウント先のフォルダをVSCode/Zedで開く",
      ],
    },
  ];

  return (
    <div className="card">
      <div className="card-header">
        <div className="card-title">
          <FontAwesomeIcon icon={faListCheck} />
          接続方法
        </div>
      </div>

      <div style={{ display: "flex", flexDirection: "column", gap: 20 }}>
        {steps.map((step, idx) => (
          <div key={idx} style={{ display: "flex", gap: 16 }}>
            <div
              style={{
                width: 40,
                height: 40,
                borderRadius: 12,
                display: "flex",
                alignItems: "center",
                justifyContent: "center",
                flexShrink: 0,
                background: "linear-gradient(135deg, rgba(126, 184, 216, 0.2) 0%, rgba(126, 184, 216, 0.08) 100%)",
                border: "1px solid rgba(126, 184, 216, 0.2)",
              }}
            >
              <FontAwesomeIcon icon={step.icon} style={{ color: "#7eb8d8" }} />
            </div>
            <div style={{ flex: 1 }}>
              <h3
                style={{
                  fontSize: 14,
                  fontWeight: 700,
                  color: "#4a7c9b",
                  marginBottom: 8,
                }}
              >
                {idx + 1}. {step.title}
              </h3>
              <ol style={{ display: "flex", flexDirection: "column", gap: 4 }}>
                {step.items.map((item, i) => (
                  <li
                    key={i}
                    style={{
                      fontSize: 13,
                      color: "#4a6b7c",
                      display: "flex",
                      gap: 8,
                    }}
                  >
                    <span style={{ color: "#7eb8d8", fontWeight: 600, minWidth: 16 }}>{i + 1}.</span>
                    {item}
                  </li>
                ))}
              </ol>
            </div>
          </div>
        ))}
      </div>

      {/* 注意 */}
      <div className="alert alert-warning" style={{ marginTop: 20, marginBottom: 0 }}>
        <FontAwesomeIcon icon={faTriangleExclamation} />
        <span>Windows 2台以上の同時接続に対応。各PCでHTMLを開いてフォルダを共有してください。</span>
      </div>
    </div>
  );
}
