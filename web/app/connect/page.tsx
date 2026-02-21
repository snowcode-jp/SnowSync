// SNOWCODE - ソフトウェア製品
// (C) SNOWCODE
// 開発者: 雪符しき
// https://snowcode.jp
// 問い合わせ: info@snowcode.jp

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
      if (!res.ok) throw new Error("Cannot connect to server");
      const html = await res.text();
      const blob = new Blob([html], { type: "text/html" });
      const a = document.createElement("a");
      a.href = URL.createObjectURL(blob);
      a.download = "ljc-connect.html";
      a.click();
      URL.revokeObjectURL(a.href);
    } catch {
      alert("Cannot connect to server. Please check that the Rust server is running.");
    }
  };

  return (
    <div>
      {/* Page Header */}
      <div className="page-header">
        <h1 className="page-title">
          <FontAwesomeIcon icon={faSnowflake} style={{ color: "#7eb8d8", fontSize: 22 }} />
          <FontAwesomeIcon icon={faPlug} style={{ color: "#7eb8d8", fontSize: 22 }} />
          Connect from Windows PC
        </h1>
      </div>

      {/* Main card */}
      <div className="card">
        {/* Download section */}
        <div className="card-header">
          <div className="card-title">
            <FontAwesomeIcon icon={faDownload} />
            Download Connection HTML
          </div>
        </div>
        <p style={{ fontSize: 14, color: "#4a6b7c", marginBottom: 20, lineHeight: 1.8 }}>
          Download the connection HTML using the button below and open it in
          <strong style={{ color: "#4a7c9b" }}> Chrome </strong>or
          <strong style={{ color: "#4a7c9b" }}> Edge </strong>on your Windows PC.
          The Mac server IP address (
          <code style={{ color: "#5a9fc8", fontFamily: "monospace", fontWeight: 600 }}>{ip}</code>
          ) is automatically embedded.
        </p>
        <button onClick={handleDownload} className="btn">
          <FontAwesomeIcon icon={faDownload} />
          Download Connection HTML
        </button>
      </div>

      {/* How to use card */}
      <div className="card">
        <div className="card-header">
          <div className="card-title">
            <FontAwesomeIcon icon={faListOl} />
            How to Use
          </div>
        </div>
        <ol style={{ display: "flex", flexDirection: "column", gap: 12 }}>
          {[
            <>Download <code style={{ color: "#5a9fc8", fontFamily: "monospace" }}>ljc-connect.html</code> using the button above</>,
            <>Copy the HTML file to your Windows PC (via USB, email, shared folder, etc.)</>,
            <>Open Chrome/Edge on Windows PC and double-click the HTML file</>,
            <>Click &quot;Select folder to connect&quot; and choose the folder to share</>,
            <>Keep the browser tab <strong style={{ color: "#4a7c9b" }}>open</strong> (closing it will disconnect)</>,
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

      {/* Why is the HTML file needed? */}
      <div className="card">
        <div className="card-header">
          <div className="card-title">
            <FontAwesomeIcon icon={faCircleQuestion} />
            Why is the HTML file needed?
          </div>
        </div>
        <p style={{ fontSize: 14, color: "#4a6b7c", lineHeight: 1.8 }}>
          For security reasons, the File System Access API only works on
          <code style={{ color: "#5a9fc8", fontFamily: "monospace", fontWeight: 600 }}> https://</code> or
          <code style={{ color: "#5a9fc8", fontFamily: "monospace", fontWeight: 600 }}> file://</code> protocols.
          It cannot be used over
          <code style={{ color: "#5a9fc8", fontFamily: "monospace", fontWeight: 600 }}> http://</code> on a local network,
          so the HTML file must be downloaded and opened via
          <code style={{ color: "#5a9fc8", fontFamily: "monospace", fontWeight: 600 }}> file://</code>.
        </p>
      </div>

      {/* Note */}
      <div className="alert alert-warning">
        <FontAwesomeIcon icon={faTriangleExclamation} />
        <span>
          Multiple Windows PCs can connect simultaneously. Open the HTML file on each PC and share folders.
          All clients can be managed from the Mac dashboard.
        </span>
      </div>
    </div>
  );
}
