// SNOWCODE - ソフトウェア製品
// (C) SNOWCODE
// 開発者: 雪符しき
// https://snowcode.jp
// 問い合わせ: info@snowcode.jp

"use client";

import { useEffect, useState } from "react";
import type { ConnectedClient } from "@/lib/types";
import { useAuth } from "@/components/AuthProvider";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import {
  faDesktop,
  faServer,
  faNetworkWired,
  faHardDrive,
  faUsers,
  faSnowflake,
} from "@fortawesome/free-solid-svg-icons";

export function ServerStatusCard() {
  const [clients, setClients] = useState<ConnectedClient[]>([]);
  const [ip, setIp] = useState("...");
  const { authHeaders } = useAuth();

  const [webPort, setWebPort] = useState("17100");
  const [rustPort, setRustPort] = useState("17200");

  useEffect(() => {
    setIp(window.location.hostname);
    const wp = window.location.port || "17100";
    setWebPort(wp);
    // Rust server port = web port + 100 (17100 -> 17200)
    const rp = String(Number(wp) + 100);
    setRustPort(rp);

    const fetchClients = async () => {
      const res = await fetch("/api/clients", { headers: authHeaders() });
      if (res.ok) setClients(await res.json());
    };
    fetchClients();
    const interval = setInterval(fetchClients, 5000);
    return () => clearInterval(interval);
  }, []);

  const httpsPort = String(Number(rustPort) + 1);

  const cards = [
    {
      icon: faDesktop,
      label: "Dashboard",
      value: `http://${ip}:${webPort}`,
    },
    {
      icon: faServer,
      label: "Relay Server",
      value: `ws://${ip}:${rustPort}/ws`,
    },
    {
      icon: faNetworkWired,
      label: "Connect URL (Windows)",
      value: `http://${ip}:${webPort}/connect`,
    },
    {
      icon: faHardDrive,
      label: "WebDAV Mount",
      value: `https://${ip}:${httpsPort}/webdav/...`,
    },
    {
      icon: faUsers,
      label: "Connected Clients",
      value: `${clients.length}`,
    },
  ];

  return (
    <div className="stats-grid">
      {cards.map((card) => (
        <div key={card.label} className="stat-card">
          {/* Snowflake watermark */}
          <div
            style={{
              position: "absolute",
              right: 10,
              top: 10,
              fontSize: 40,
              color: "rgba(126, 184, 216, 0.15)",
            }}
          >
            <FontAwesomeIcon icon={faSnowflake} />
          </div>
          <div className="stat-icon">
            <FontAwesomeIcon icon={card.icon} />
          </div>
          <div className="stat-value" style={{ fontSize: card.label === "Connected Clients" ? 36 : 18 }}>
            {card.label === "Connected Clients" ? card.value : ""}
          </div>
          {card.label !== "Connected Clients" && (
            <p
              style={{
                fontSize: 12,
                fontFamily: "monospace",
                color: "#4a6b7c",
                wordBreak: "break-all",
                lineHeight: 1.5,
              }}
            >
              {card.value}
            </p>
          )}
          <div className="stat-label">{card.label}</div>
        </div>
      ))}
    </div>
  );
}
