// SNOWCODE - ソフトウェア製品
// (C) SNOWCODE
// 開発者: 雪符しき
// https://snowcode.jp
// 問い合わせ: info@snowcode.jp

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
      title: "Windows PC (Share Folders)",
      items: [
        "Download the connection HTML from the Connect page",
        "Copy the HTML file to your Windows PC",
        "Open it in Chrome / Edge",
        "Click \"Select folder to connect\"",
        "Keep the browser tab open",
      ],
    },
    {
      icon: faGlobe,
      title: "Mac (Browse Files in Browser)",
      items: [
        "Go to File Browser or click Browse from the client list",
        "Select a connected client",
        "Browse, download, upload, delete, and rename files",
      ],
    },
    {
      icon: faHardDrive,
      title: "Mac (Finder / VSCode / Zed)",
      items: [
        "Click \"Open in Finder\" from the client list",
        "Mounts via HTTPS (self-signed certificate) in Finder",
        "Open the mounted folder in VSCode/Zed",
      ],
    },
  ];

  return (
    <div className="card">
      <div className="card-header">
        <div className="card-title">
          <FontAwesomeIcon icon={faListCheck} />
          Connection Guide
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

      {/* Note */}
      <div className="alert alert-warning" style={{ marginTop: 20, marginBottom: 0 }}>
        <FontAwesomeIcon icon={faTriangleExclamation} />
        <span>Supports multiple Windows PCs simultaneously. Open the HTML file on each PC to share folders.</span>
      </div>
    </div>
  );
}
