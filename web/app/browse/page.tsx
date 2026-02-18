// SNOWCODE - ソフトウェア製品
// (C) SNOWCODE
// 開発者: 雪符しき
// https://snowcode.jp
// 問い合わせ: info@snowcode.jp
// 本ソフトウェアは利用権の販売であり、著作権はSNOWCODEに帰属します。
// 署名の削除・改変は禁止されています。

"use client";

import { Suspense, useEffect, useState } from "react";
import { useSearchParams } from "next/navigation";
import { RemoteBrowser } from "@/components/RemoteBrowser";
import type { ConnectedClient } from "@/lib/types";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faFolderOpen, faSpinner, faSnowflake } from "@fortawesome/free-solid-svg-icons";

function BrowseContent() {
  const searchParams = useSearchParams();
  const [clients, setClients] = useState<ConnectedClient[]>([]);
  const [selected, setSelected] = useState<string | null>(
    searchParams.get("client")
  );

  useEffect(() => {
    const fetchClients = async () => {
      const res = await fetch("/api/clients");
      if (res.ok) {
        const data: ConnectedClient[] = await res.json();
        setClients(data);
        if (data.length > 0 && !selected) {
          setSelected(data[0].id);
        }
      }
    };
    fetchClients();
    const interval = setInterval(fetchClients, 5000);
    return () => clearInterval(interval);
  }, []);

  const selectedClient = clients.find((c) => c.id === selected);

  return (
    <div>
      {/* Page Header */}
      <div className="page-header">
        <div style={{ display: "flex", alignItems: "center", gap: 16 }}>
          <h1 className="page-title">
            <FontAwesomeIcon icon={faSnowflake} style={{ color: "#7eb8d8", fontSize: 22 }} />
            <FontAwesomeIcon icon={faFolderOpen} style={{ color: "#7eb8d8", fontSize: 22 }} />
            リモートファイルブラウザ
          </h1>
          {selectedClient && (
            <p style={{ fontSize: 13, color: "#7eb8d8", fontWeight: 500 }}>
              閲覧中: {selectedClient.folder_name} ({selectedClient.name})
            </p>
          )}
        </div>
        {clients.length > 0 && (
          <select
            value={selected ?? ""}
            onChange={(e) => setSelected(e.target.value)}
            className="form-input"
            style={{ width: "auto", padding: "10px 18px" }}
          >
            {clients.map((c) => (
              <option key={c.id} value={c.id}>
                {c.name} - {c.folder_name}
              </option>
            ))}
          </select>
        )}
      </div>

      {selected ? (
        <RemoteBrowser key={selected} clientId={selected} />
      ) : (
        <div className="card" style={{ textAlign: "center", padding: "40px 25px" }}>
          <FontAwesomeIcon
            icon={faFolderOpen}
            style={{ fontSize: 32, color: "rgba(126, 184, 216, 0.4)", marginBottom: 12 }}
          />
          <p style={{ color: "#4a7c9b" }}>
            接続中のクライアントがありません。Windows PCで接続用HTMLを開いてフォルダを共有してください。
          </p>
        </div>
      )}
    </div>
  );
}

export default function BrowsePage() {
  return (
    <Suspense
      fallback={
        <div style={{ padding: 30, color: "#7eb8d8" }}>
          <FontAwesomeIcon icon={faSpinner} spin style={{ marginRight: 8 }} />
          読み込み中...
        </div>
      }
    >
      <BrowseContent />
    </Suspense>
  );
}
