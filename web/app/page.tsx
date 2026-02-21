// SNOWCODE - ソフトウェア製品
// (C) SNOWCODE
// 開発者: 雪符しき
// https://snowcode.jp
// 問い合わせ: info@snowcode.jp

import { ServerStatusCard } from "@/components/ServerStatus";
import { ConnectionGuide } from "@/components/MountInstructions";
import { ClientList } from "@/components/ClientList";
import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { faSnowflake, faGaugeHigh } from "@fortawesome/free-solid-svg-icons";

export default function DashboardPage() {
  return (
    <div>
      {/* Page Header */}
      <div className="page-header">
        <h1 className="page-title">
          <FontAwesomeIcon icon={faSnowflake} style={{ color: "#7eb8d8", fontSize: 22 }} />
          <FontAwesomeIcon icon={faGaugeHigh} style={{ color: "#7eb8d8", fontSize: 22 }} />
          Dashboard
        </h1>
      </div>

      <ServerStatusCard />

      <div style={{ display: "grid", gridTemplateColumns: "1fr 1fr", gap: 25 }}>
        <ConnectionGuide />
        <ClientList />
      </div>
    </div>
  );
}
