// SNOWCODE - ソフトウェア製品
// (C) SNOWCODE
// 開発者: 雪符しき
// https://snowcode.jp
// 問い合わせ: info@snowcode.jp
// 本ソフトウェアは利用権の販売であり、著作権はSNOWCODEに帰属します。
// 署名の削除・改変は禁止されています。

"use client";

import Link from "next/link";
import { usePathname } from "next/navigation";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import {
  faGaugeHigh,
  faFolderOpen,
  faPlug,
  faSnowflake,
  faChevronDown,
} from "@fortawesome/free-solid-svg-icons";

const navItems = [
  { href: "/", label: "ダッシュボード", icon: faGaugeHigh },
  { href: "/browse", label: "ファイル閲覧", icon: faFolderOpen },
  { href: "/connect", label: "接続", icon: faPlug },
];

export function Sidebar() {
  const pathname = usePathname();

  return (
    <aside
      style={{
        width: 260,
        background: "linear-gradient(180deg, rgba(255,255,255,0.95) 0%, rgba(232,244,252,0.95) 100%)",
        backdropFilter: "blur(10px)",
        borderRight: "2px solid rgba(126, 184, 216, 0.3)",
        padding: 0,
        position: "sticky",
        top: 0,
        height: "100vh",
        overflowY: "auto",
        boxShadow: "5px 0 20px rgba(100, 150, 200, 0.1)",
        display: "flex",
        flexDirection: "column",
        flexShrink: 0,
      }}
    >
      {/* Header */}
      <div
        style={{
          padding: "25px 20px",
          background: "linear-gradient(135deg, #7eb8d8 0%, #5a9fc8 100%)",
          color: "#fff",
          position: "relative",
          overflow: "hidden",
        }}
      >
        <div
          style={{
            position: "absolute",
            right: -10,
            top: -10,
            fontSize: 80,
            opacity: 0.15,
            transform: "rotate(15deg)",
          }}
        >
          <FontAwesomeIcon icon={faSnowflake} />
        </div>
        <h1
          style={{
            fontSize: 18,
            fontWeight: 700,
            letterSpacing: 1,
            position: "relative",
            zIndex: 1,
          }}
        >
          <FontAwesomeIcon icon={faSnowflake} style={{ marginRight: 8 }} />
          LocalJackControl
        </h1>
        <p
          style={{
            fontSize: 12,
            opacity: 0.9,
            marginTop: 5,
            position: "relative",
            zIndex: 1,
          }}
        >
          管理ダッシュボード
        </p>
      </div>

      {/* Nav */}
      <nav style={{ flex: 1 }}>
        <div
          style={{
            padding: "15px 0",
            borderBottom: "1px solid rgba(126, 184, 216, 0.2)",
          }}
        >
          <div
            style={{
              padding: "8px 20px",
              fontSize: 11,
              textTransform: "uppercase",
              letterSpacing: 2,
              color: "#7eb8d8",
              fontWeight: 700,
              display: "flex",
              alignItems: "center",
              gap: 8,
            }}
          >
            <FontAwesomeIcon icon={faSnowflake} style={{ fontSize: 10 }} />
            メイン
          </div>
          {navItems.map((item) => {
            const isActive =
              item.href === "/" ? pathname === "/" : pathname.startsWith(item.href);
            return (
              <Link
                key={item.href}
                href={item.href}
                style={{
                  display: "flex",
                  alignItems: "center",
                  gap: 12,
                  padding: "12px 20px",
                  color: isActive ? "#4a7c9b" : "#4a6b7c",
                  textDecoration: "none",
                  transition: "all 0.3s ease",
                  fontSize: 14,
                  borderLeft: `3px solid ${isActive ? "#5a9fc8" : "transparent"}`,
                  background: isActive
                    ? "linear-gradient(90deg, rgba(126, 184, 216, 0.25) 0%, transparent 100%)"
                    : "transparent",
                  fontWeight: isActive ? 600 : 400,
                }}
              >
                <FontAwesomeIcon
                  icon={item.icon}
                  fixedWidth
                  style={{
                    width: 20,
                    textAlign: "center",
                    color: "#7eb8d8",
                    transition: "transform 0.3s ease",
                  }}
                />
                {item.label}
              </Link>
            );
          })}
        </div>
      </nav>

      {/* Footer */}
      <div
        style={{
          padding: 20,
          borderTop: "1px solid rgba(126, 184, 216, 0.2)",
          background: "rgba(126, 184, 216, 0.08)",
        }}
      >
        <p style={{ fontSize: 13, color: "#5a9fc8", fontWeight: 500 }}>
          <FontAwesomeIcon icon={faSnowflake} style={{ marginRight: 8, fontSize: 12 }} />
          v1.0 - LAN専用
        </p>
      </div>
    </aside>
  );
}
