// SNOWCODE - ソフトウェア製品
// (C) SNOWCODE
// 開発者: 雪符しき
// https://snowcode.jp
// 問い合わせ: info@snowcode.jp

"use client";

import { createContext, useContext, useState, useEffect, useCallback } from "react";

interface AuthContextType {
  token: string;
  setToken: (token: string) => void;
  isAuthenticated: boolean;
  authHeaders: () => Record<string, string>;
}

const AuthContext = createContext<AuthContextType>({
  token: "",
  setToken: () => {},
  isAuthenticated: false,
  authHeaders: () => ({}),
});

export function useAuth() {
  return useContext(AuthContext);
}

const STORAGE_KEY = "snowsync_api_token";

export function AuthProvider({ children }: { children: React.ReactNode }) {
  const [token, setTokenState] = useState("");
  const [ready, setReady] = useState(false);

  useEffect(() => {
    const saved = localStorage.getItem(STORAGE_KEY) || "";
    setTokenState(saved);
    setReady(true);
  }, []);

  const setToken = useCallback((t: string) => {
    setTokenState(t);
    if (t) {
      localStorage.setItem(STORAGE_KEY, t);
    } else {
      localStorage.removeItem(STORAGE_KEY);
    }
  }, []);

  const authHeaders = useCallback((): Record<string, string> => {
    if (!token) return {};
    return { Authorization: `Bearer ${token}` };
  }, [token]);

  if (!ready) return null;

  if (!token) {
    return <TokenInput onSubmit={setToken} />;
  }

  return (
    <AuthContext.Provider value={{ token, setToken, isAuthenticated: !!token, authHeaders }}>
      {children}
    </AuthContext.Provider>
  );
}

function TokenInput({ onSubmit }: { onSubmit: (token: string) => void }) {
  const [value, setValue] = useState("");

  return (
    <div
      style={{
        display: "flex",
        alignItems: "center",
        justifyContent: "center",
        minHeight: "100vh",
        background: "linear-gradient(135deg, #0a0e27, #1a1040)",
      }}
    >
      <div
        style={{
          background: "rgba(255,255,255,0.06)",
          borderRadius: 16,
          padding: "40px 36px",
          maxWidth: 400,
          width: "100%",
          textAlign: "center",
          border: "1px solid rgba(255,255,255,0.1)",
        }}
      >
        <h2 style={{ color: "#e8f4fd", marginBottom: 8, fontSize: 20 }}>
          ❄ SnowSync
        </h2>
        <p style={{ color: "rgba(255,255,255,0.5)", fontSize: 13, marginBottom: 24 }}>
          サーバー起動時に表示された API Token を入力してください
        </p>
        <input
          type="text"
          placeholder="API Token"
          value={value}
          onChange={(e) => setValue(e.target.value)}
          onKeyDown={(e) => {
            if (e.key === "Enter" && value.trim()) onSubmit(value.trim());
          }}
          style={{
            width: "100%",
            padding: "10px 14px",
            borderRadius: 8,
            border: "1px solid rgba(255,255,255,0.15)",
            background: "rgba(0,0,0,0.3)",
            color: "#e8f4fd",
            fontSize: 14,
            outline: "none",
            marginBottom: 16,
            boxSizing: "border-box",
          }}
        />
        <button
          onClick={() => value.trim() && onSubmit(value.trim())}
          style={{
            width: "100%",
            padding: "10px 0",
            borderRadius: 8,
            border: "none",
            background: "linear-gradient(135deg, #4a90d9, #7eb8d8)",
            color: "#fff",
            fontWeight: 600,
            fontSize: 14,
            cursor: "pointer",
          }}
        >
          接続
        </button>
      </div>
    </div>
  );
}
