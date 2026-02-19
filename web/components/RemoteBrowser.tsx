// SNOWCODE - ソフトウェア製品
// (C) SNOWCODE
// 開発者: 雪符しき
// https://snowcode.jp
// 問い合わせ: info@snowcode.jp

"use client";

import { useEffect, useState, useCallback } from "react";
import type { FileEntry } from "@/lib/types";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import {
  faFolder,
  faFile,
  faArrowUp,
  faCloudArrowUp,
  faFolderPlus,
  faArrowsRotate,
  faDownload,
  faPenToSquare,
  faTrash,
  faSpinner,
  faFolderOpen,
  faCircleExclamation,
} from "@fortawesome/free-solid-svg-icons";
import { useToast } from "@/components/Toast";

interface RemoteBrowserProps {
  clientId: string;
}

function formatSize(bytes: number): string {
  if (bytes === 0) return "-";
  const units = ["B", "KB", "MB", "GB"];
  let i = 0;
  let size = bytes;
  while (size >= 1024 && i < units.length - 1) {
    size /= 1024;
    i++;
  }
  return `${size.toFixed(i > 0 ? 1 : 0)} ${units[i]}`;
}

function formatDate(iso: string): string {
  return new Date(iso).toLocaleString("ja-JP", {
    year: "numeric",
    month: "2-digit",
    day: "2-digit",
    hour: "2-digit",
    minute: "2-digit",
  });
}

async function relayCommand(clientId: string, cmd: Record<string, unknown>) {
  const res = await fetch(`/api/relay/${clientId}`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(cmd),
  });
  const data = await res.json();
  if (!data.ok) throw new Error(data.error || "操作に失敗しました");
  return data.data;
}

export function RemoteBrowser({ clientId }: RemoteBrowserProps) {
  const [files, setFiles] = useState<FileEntry[]>([]);
  const [currentPath, setCurrentPath] = useState<string[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [actionMsg, setActionMsg] = useState<string | null>(null);
  const { showToast } = useToast();

  const pathStr = "/" + currentPath.join("/");

  const fetchFiles = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const data = await relayCommand(clientId, {
        type: "readdir",
        path: pathStr,
      });
      setFiles(data as FileEntry[]);
    } catch (err: unknown) {
      setError(err instanceof Error ? err.message : "読み込みに失敗しました");
    } finally {
      setLoading(false);
    }
  }, [clientId, pathStr]);

  useEffect(() => {
    fetchFiles();
  }, [fetchFiles]);

  const navigateTo = (dirName: string) => {
    setCurrentPath([...currentPath, dirName]);
  };

  const navigateUp = () => {
    setCurrentPath(currentPath.slice(0, -1));
  };

  const handleDownload = async (fileName: string) => {
    setActionMsg(`${fileName} をダウンロード中...`);
    try {
      const filePath = [...currentPath, fileName].join("/");
      const result = (await relayCommand(clientId, {
        type: "readFile",
        path: "/" + filePath,
      })) as { data: string; name: string; type: string };

      const binary = atob(result.data);
      const bytes = new Uint8Array(binary.length);
      for (let i = 0; i < binary.length; i++) {
        bytes[i] = binary.charCodeAt(i);
      }
      const blob = new Blob([bytes], {
        type: result.type || "application/octet-stream",
      });
      const url = URL.createObjectURL(blob);
      const a = document.createElement("a");
      a.href = url;
      a.download = result.name;
      a.click();
      URL.revokeObjectURL(url);
      setActionMsg(null);
      showToast("success", `${fileName} をダウンロードしました`);
    } catch (err: unknown) {
      setActionMsg(null);
      showToast("error", `ダウンロード失敗: ${err instanceof Error ? err.message : "エラー"}`);
    }
  };

  const handleUpload = async () => {
    const input = document.createElement("input");
    input.type = "file";
    input.multiple = true;
    input.onchange = async () => {
      if (!input.files) return;
      for (const file of Array.from(input.files)) {
        setActionMsg(`${file.name} をアップロード中...`);
        try {
          const buffer = await file.arrayBuffer();
          const bytes = new Uint8Array(buffer);
          let binary = "";
          for (let i = 0; i < bytes.length; i++) {
            binary += String.fromCharCode(bytes[i]);
          }
          const filePath = [...currentPath, file.name].join("/");
          await relayCommand(clientId, {
            type: "writeFile",
            path: "/" + filePath,
            data: btoa(binary),
          });
        } catch (err: unknown) {
          setActionMsg(null);
          showToast("error", `アップロード失敗: ${err instanceof Error ? err.message : "エラー"}`);
          return;
        }
      }
      setActionMsg(null);
      showToast("success", "アップロードが完了しました");
      fetchFiles();
    };
    input.click();
  };

  const handleNewFolder = async () => {
    const name = prompt("新しいフォルダ名:");
    if (!name) return;
    try {
      const folderPath = [...currentPath, name].join("/");
      await relayCommand(clientId, {
        type: "mkdir",
        path: "/" + folderPath,
      });
      showToast("success", `フォルダ「${name}」を作成しました`);
      fetchFiles();
    } catch (err: unknown) {
      showToast("error", `フォルダ作成失敗: ${err instanceof Error ? err.message : "エラー"}`);
    }
  };

  const handleDelete = async (name: string, isDir: boolean) => {
    if (!confirm(`${isDir ? "フォルダ" : "ファイル"}「${name}」を削除しますか?`)) return;
    try {
      const targetPath = [...currentPath, name].join("/");
      await relayCommand(clientId, {
        type: "delete",
        path: "/" + targetPath,
      });
      showToast("success", `「${name}」を削除しました`);
      fetchFiles();
    } catch (err: unknown) {
      showToast("error", `削除失敗: ${err instanceof Error ? err.message : "エラー"}`);
    }
  };

  const handleRename = async (oldName: string) => {
    const newName = prompt("新しい名前:", oldName);
    if (!newName || newName === oldName) return;
    try {
      const oldPath = "/" + [...currentPath, oldName].join("/");
      const newPath = "/" + [...currentPath, newName].join("/");
      await relayCommand(clientId, {
        type: "rename",
        oldPath,
        newPath,
      });
      showToast("success", `「${oldName}」→「${newName}」にリネームしました`);
      fetchFiles();
    } catch (err: unknown) {
      showToast("error", `リネーム失敗: ${err instanceof Error ? err.message : "エラー"}`);
    }
  };

  const breadcrumbs = ["root", ...currentPath];

  return (
    <div className="card" style={{ padding: 0, overflow: "hidden" }}>
      {/* Toolbar */}
      <div
        style={{
          padding: "14px 20px",
          display: "flex",
          alignItems: "center",
          justifyContent: "space-between",
          borderBottom: "2px solid rgba(126, 184, 216, 0.15)",
          background: "linear-gradient(135deg, rgba(126, 184, 216, 0.08) 0%, transparent 100%)",
        }}
      >
        {/* Breadcrumbs */}
        <div style={{ display: "flex", alignItems: "center", gap: 6, fontSize: 14 }}>
          {breadcrumbs.map((segment, i) => (
            <span key={i} style={{ display: "flex", alignItems: "center", gap: 6 }}>
              {i > 0 && <span style={{ color: "#a8c8dc" }}>/</span>}
              <button
                onClick={() => setCurrentPath(currentPath.slice(0, i))}
                style={{
                  background: "none",
                  border: "none",
                  cursor: "pointer",
                  fontFamily: "inherit",
                  fontSize: 14,
                  color: i === breadcrumbs.length - 1 ? "#4a7c9b" : "#7eb8d8",
                  fontWeight: i === breadcrumbs.length - 1 ? 700 : 500,
                  transition: "color 0.2s ease",
                  padding: 0,
                }}
              >
                {segment}
              </button>
            </span>
          ))}
        </div>

        {/* Action buttons */}
        <div style={{ display: "flex", gap: 8 }}>
          <button onClick={handleUpload} className="btn btn-sm">
            <FontAwesomeIcon icon={faCloudArrowUp} />
            アップロード
          </button>
          <button onClick={handleNewFolder} className="btn btn-sm btn-secondary">
            <FontAwesomeIcon icon={faFolderPlus} />
            新規フォルダ
          </button>
          <button onClick={fetchFiles} className="btn btn-sm btn-secondary">
            <FontAwesomeIcon icon={faArrowsRotate} />
            更新
          </button>
        </div>
      </div>

      {/* Action message (progress indicator) */}
      {actionMsg && (
        <div className="alert alert-info" style={{ margin: 0, borderRadius: 0 }}>
          <FontAwesomeIcon icon={faSpinner} spin />
          {actionMsg}
        </div>
      )}

      {/* Content */}
      {loading ? (
        <div style={{ padding: 40, textAlign: "center", color: "#7eb8d8" }}>
          <FontAwesomeIcon icon={faSpinner} spin style={{ marginRight: 8 }} />
          読み込み中...
        </div>
      ) : error ? (
        <div style={{ padding: 40, textAlign: "center" }}>
          <FontAwesomeIcon icon={faCircleExclamation} style={{ color: "#ff6b7a", fontSize: 24, marginBottom: 8, display: "block" }} />
          <p style={{ color: "#c62828" }}>{error}</p>
        </div>
      ) : files.length === 0 ? (
        <div style={{ padding: 40, textAlign: "center" }}>
          <FontAwesomeIcon icon={faFolderOpen} style={{ color: "rgba(126, 184, 216, 0.4)", fontSize: 24, marginBottom: 8, display: "block" }} />
          <p style={{ color: "#7eb8d8" }}>空のディレクトリ</p>
        </div>
      ) : (
        <div className="table-wrapper">
          <table className="table">
            <thead>
              <tr>
                <th>名前</th>
                <th style={{ textAlign: "right" }}>サイズ</th>
                <th style={{ textAlign: "right" }}>更新日時</th>
                <th style={{ textAlign: "right" }}>操作</th>
              </tr>
            </thead>
            <tbody>
              {currentPath.length > 0 && (
                <tr
                  onClick={navigateUp}
                  style={{ cursor: "pointer" }}
                >
                  <td>
                    <div style={{ display: "flex", alignItems: "center", gap: 10 }}>
                      <FontAwesomeIcon icon={faArrowUp} style={{ color: "#7eb8d8", width: 16 }} />
                      <span style={{ color: "#7eb8d8", fontWeight: 500 }}>..</span>
                    </div>
                  </td>
                  <td></td>
                  <td></td>
                  <td></td>
                </tr>
              )}
              {files.map((file) => (
                <tr key={file.name}>
                  <td
                    style={{ cursor: file.is_dir ? "pointer" : "default" }}
                    onClick={() => file.is_dir && navigateTo(file.name)}
                  >
                    <div style={{ display: "flex", alignItems: "center", gap: 10 }}>
                      <FontAwesomeIcon
                        icon={file.is_dir ? faFolder : faFile}
                        style={{
                          color: file.is_dir ? "#5a9fc8" : "#a8c8dc",
                          width: 16,
                        }}
                      />
                      <span style={{
                        color: file.is_dir ? "#4a7c9b" : "#4a6b7c",
                        fontWeight: file.is_dir ? 600 : 400,
                      }}>
                        {file.name}
                      </span>
                    </div>
                  </td>
                  <td style={{ textAlign: "right", fontFamily: "monospace", fontSize: 12, color: "#7eb8d8" }}>
                    {file.is_dir ? "-" : formatSize(file.size)}
                  </td>
                  <td style={{ textAlign: "right", fontSize: 12, color: "#a8c8dc" }}>
                    {formatDate(file.modified)}
                  </td>
                  <td style={{ textAlign: "right" }}>
                    <div style={{ display: "flex", gap: 6, justifyContent: "flex-end" }}>
                      {!file.is_dir && (
                        <button
                          onClick={() => handleDownload(file.name)}
                          className="btn btn-sm"
                          style={{ padding: "5px 12px", fontSize: 11 }}
                        >
                          <FontAwesomeIcon icon={faDownload} />
                          DL
                        </button>
                      )}
                      <button
                        onClick={() => handleRename(file.name)}
                        className="btn btn-sm btn-secondary"
                        style={{ padding: "5px 12px", fontSize: 11 }}
                      >
                        <FontAwesomeIcon icon={faPenToSquare} />
                        リネーム
                      </button>
                      <button
                        onClick={() => handleDelete(file.name, file.is_dir)}
                        className="btn btn-sm btn-danger"
                        style={{ padding: "5px 12px", fontSize: 11 }}
                      >
                        <FontAwesomeIcon icon={faTrash} />
                        削除
                      </button>
                    </div>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}
    </div>
  );
}
