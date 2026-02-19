// SNOWCODE - ソフトウェア製品
// (C) SNOWCODE
// 開発者: 雪符しき
// https://snowcode.jp
// 問い合わせ: info@snowcode.jp

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default = "default_bind")]
    pub bind_address: String,
}

fn default_port() -> u16 {
    17200
}
fn default_bind() -> String {
    "0.0.0.0".to_string()
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            port: default_port(),
            bind_address: default_bind(),
        }
    }
}

impl AppConfig {
    /// Load config from environment variables, falling back to defaults.
    /// - LJC_PORT: server port (default: 17200)
    /// - LJC_BIND: bind address (default: 0.0.0.0)
    pub fn from_env() -> Self {
        let port = std::env::var("LJC_PORT")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or_else(default_port);
        let bind_address = std::env::var("LJC_BIND")
            .unwrap_or_else(|_| default_bind());
        Self { port, bind_address }
    }
}
