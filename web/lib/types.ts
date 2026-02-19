// SNOWCODE - ソフトウェア製品
// (C) SNOWCODE
// 開発者: 雪符しき
// https://snowcode.jp
// 問い合わせ: info@snowcode.jp

export interface ConnectedClient {
  id: string;
  name: string;
  folder_name: string;
  connected_at: string;
}

export interface FileEntry {
  name: string;
  is_dir: boolean;
  size: number;
  modified: string;
}

export interface RelayCommand {
  type: "readdir" | "readFile" | "writeFile" | "mkdir" | "delete" | "rename" | "stat";
  path?: string;
  data?: string;
  oldPath?: string;
  newPath?: string;
}

export interface RelayResponse {
  id: string;
  ok: boolean;
  data?: unknown;
  error?: string;
}
