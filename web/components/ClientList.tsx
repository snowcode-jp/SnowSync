// SNOWCODE - ソフトウェア製品
// (C) SNOWCODE
// 開発者: 雪符しき
// https://snowcode.jp
// 問い合わせ: info@snowcode.jp

"use client";

import { useEffect, useState, useCallback } from "react";
import type { ConnectedClient } from "@/lib/types";
import Link from "next/link";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import {
  faCircle,
  faEye,
  faFolderOpen,
  faEject,
  faFolderPlus,
  faComputer,
  faUsers,
} from "@fortawesome/free-solid-svg-icons";
import { useToast } from "@/components/Toast";

interface MountInfo {
  url: string;
  mount_point: string;
  raw: string;
}

export function ClientList() {
  const [clients, setClients] = useState<ConnectedClient[]>([]);
  const [ip, setIp] = useState("...");
  const [mountPath, setMountPath] = useState("~/Public/mount");
  const [mounting, setMounting] = useState<Record<string, boolean>>({});
  const [mounts, setMounts] = useState<MountInfo[]>([]);
  const { showToast } = useToast();

  const fetchClients = useCallback(async () => {
    const res = await fetch("/api/clients");
    if (res.ok) setClients(await res.json());
  }, []);

  const fetchMounts = useCallback(async () => {
    try {
      const res = await fetch("/api/mounts");
      if (res.ok) setMounts(await res.json());
    } catch {
      // ignore
    }
  }, []);

  useEffect(() => {
    setIp(window.location.hostname);
    fetchClients();
    fetchMounts();
    const interval = setInterval(() => {
      fetchClients();
      fetchMounts();
    }, 3000);
    return () => clearInterval(interval);
  }, [fetchClients, fetchMounts]);

  const isMounted = (clientId: string) => {
    return mounts.some((m) => m.url.includes(clientId) || m.mount_point.includes(clientId.slice(0, 8)));
  };

  const getMountPoint = (clientId: string) => {
    const m = mounts.find((m) => m.url.includes(clientId) || m.mount_point.includes(clientId.slice(0, 8)));
    return m?.mount_point || "";
  };

  const handleMount = async (clientId: string) => {
    setMounting((prev) => ({ ...prev, [clientId]: true }));
    try {
      const res = await fetch("/api/mount", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ client_id: clientId, mount_path: mountPath }),
      });
      const data = await res.json();
      if (res.ok && data.ok) {
        showToast("success", `マウント成功: ${data.mount_point}`);
        fetchMounts();
      } else {
        showToast("error", data.error || "マウントに失敗しました");
      }
    } catch {
      showToast("error", "サーバーとの通信に失敗しました");
    } finally {
      setMounting((prev) => ({ ...prev, [clientId]: false }));
    }
  };

  const handleUnmount = async (clientId: string) => {
    const mp = getMountPoint(clientId);
    if (!mp) return;
    setMounting((prev) => ({ ...prev, [clientId]: true }));
    try {
      const res = await fetch("/api/unmount", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ mount_path: mp }),
      });
      const data = await res.json();
      if (res.ok && data.ok) {
        showToast("success", "アンマウントしました");
        fetchMounts();
      } else {
        showToast("error", data.error || "アンマウントに失敗しました");
      }
    } catch {
      showToast("error", "サーバーとの通信に失敗しました");
    } finally {
      setMounting((prev) => ({ ...prev, [clientId]: false }));
    }
  };

  if (clients.length === 0) {
    return (
      <div className="card" style={{ textAlign: "center", padding: "40px 25px" }}>
        <FontAwesomeIcon
          icon={faComputer}
          style={{ fontSize: 32, color: "rgba(126, 184, 216, 0.4)", marginBottom: 12 }}
        />
        <p style={{ color: "#4a7c9b", fontWeight: 500 }}>接続中のクライアントはありません</p>
        <p style={{ fontSize: 12, color: "#7eb8d8", marginTop: 6 }}>
          Windows PCで接続用HTMLを開いてフォルダを共有してください
        </p>
      </div>
    );
  }

  return (
    <div>
      {/* マウントパス設定 */}
      <div className="card" style={{ marginBottom: 20 }}>
        <div style={{ display: "flex", alignItems: "center", gap: 10, marginBottom: 12 }}>
          <FontAwesomeIcon icon={faFolderPlus} style={{ color: "#7eb8d8" }} />
          <label className="form-label" style={{ marginBottom: 0 }}>
            マウント先フォルダ
          </label>
        </div>
        <input
          type="text"
          value={mountPath}
          onChange={(e) => setMountPath(e.target.value)}
          className="form-input"
          style={{ fontFamily: "monospace" }}
          placeholder="~/Public/mount"
        />
      </div>

      {/* クライアント一覧 */}
      <div className="card" style={{ padding: 0, overflow: "hidden" }}>
        <div className="card-header" style={{ padding: "18px 25px", margin: 0, borderRadius: "20px 20px 0 0" }}>
          <div className="card-title">
            <FontAwesomeIcon icon={faUsers} />
            接続クライアント
          </div>
        </div>
        <div className="table-wrapper">
          <table className="table">
            <thead>
              <tr>
                <th>状態</th>
                <th>クライアント</th>
                <th>フォルダ</th>
                <th style={{ textAlign: "right" }}>操作</th>
              </tr>
            </thead>
            <tbody>
              {clients.map((client) => {
                const mounted = isMounted(client.id);
                const mp = getMountPoint(client.id);
                return (
                  <tr key={client.id}>
                    <td>
                      <div style={{ display: "flex", alignItems: "center", gap: 8 }}>
                        <span className="status-badge status-online">
                          <FontAwesomeIcon icon={faCircle} style={{ fontSize: 6 }} />
                          接続中
                        </span>
                        {mounted && (
                          <span className="status-badge status-mounted">
                            マウント中
                          </span>
                        )}
                      </div>
                    </td>
                    <td>
                      <div style={{ fontWeight: 600, color: "#4a7c9b" }}>{client.name}</div>
                      <div style={{ fontSize: 11, fontFamily: "monospace", color: "#7eb8d8", marginTop: 2 }}>
                        {client.id.slice(0, 8)}...
                      </div>
                    </td>
                    <td>
                      <div style={{ color: "#4a6b7c" }}>{client.folder_name}</div>
                      {mounted && mp && (
                        <div style={{ fontSize: 11, fontFamily: "monospace", color: "#388e3c", marginTop: 2 }}>
                          {mp}
                        </div>
                      )}
                    </td>
                    <td style={{ textAlign: "right" }}>
                      <div style={{ display: "flex", justifyContent: "flex-end", gap: 8 }}>
                        <Link href={`/browse?client=${client.id}`} className="btn btn-sm">
                          <FontAwesomeIcon icon={faEye} />
                          閲覧
                        </Link>
                        {!mounted ? (
                          <button
                            onClick={() => handleMount(client.id)}
                            disabled={mounting[client.id]}
                            className="btn btn-sm"
                            style={{
                              background: "linear-gradient(135deg, #66bb6a 0%, #43a047 100%)",
                              boxShadow: "0 4px 15px rgba(67, 160, 71, 0.3)",
                            }}
                          >
                            <FontAwesomeIcon icon={faFolderOpen} />
                            {mounting[client.id] ? "マウント中..." : "Finderで開く"}
                          </button>
                        ) : (
                          <button
                            onClick={() => handleUnmount(client.id)}
                            disabled={mounting[client.id]}
                            className="btn btn-sm btn-danger"
                          >
                            <FontAwesomeIcon icon={faEject} />
                            アンマウント
                          </button>
                        )}
                      </div>
                    </td>
                  </tr>
                );
              })}
            </tbody>
          </table>
        </div>
      </div>
    </div>
  );
}
