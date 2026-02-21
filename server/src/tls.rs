// SNOWCODE - ソフトウェア製品
// (C) SNOWCODE
// 開発者: 雪符しき
// https://snowcode.jp
// 問い合わせ: info@snowcode.jp

use rcgen::{CertificateParams, KeyPair, SanType};
use rustls::ServerConfig;
use rustls_pki_types::{CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer};
use std::path::PathBuf;
use std::sync::Arc;

/// Directory where self-signed cert/key are cached.
fn cert_dir() -> PathBuf {
    let dir = dirs_or_home().join(".ljc-certs");
    let _ = std::fs::create_dir_all(&dir);
    // Restrict directory permissions to owner only (rwx------)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = std::fs::set_permissions(&dir, std::fs::Permissions::from_mode(0o700));
    }
    dir
}

fn dirs_or_home() -> PathBuf {
    std::env::var("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("/tmp"))
}

/// Generate (or load cached) self-signed TLS config for the given local IP.
pub fn make_tls_config(local_ip: &str) -> anyhow::Result<Arc<ServerConfig>> {
    let dir = cert_dir();
    let cert_path = dir.join("cert.pem");
    let key_path = dir.join("key.pem");

    // Check if cached cert exists and matches IP
    let ip_marker_path = dir.join("ip.txt");
    let cached_ip = std::fs::read_to_string(&ip_marker_path).unwrap_or_default();

    let (cert_pem, key_pem) = if cert_path.exists()
        && key_path.exists()
        && cached_ip.trim() == local_ip
    {
        tracing::info!("Using cached TLS certificate for {}", local_ip);
        (
            std::fs::read_to_string(&cert_path)?,
            std::fs::read_to_string(&key_path)?,
        )
    } else {
        tracing::info!("Generating self-signed TLS certificate for {}", local_ip);
        let (cert, key) = generate_self_signed(local_ip)?;
        std::fs::write(&cert_path, &cert)?;
        std::fs::write(&key_path, &key)?;
        std::fs::write(&ip_marker_path, local_ip)?;
        // Restrict private key file permissions (rw-------)
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let _ = std::fs::set_permissions(&key_path, std::fs::Permissions::from_mode(0o600));
        }
        (cert, key)
    };

    // Parse PEM into DER
    let cert_der = pem_to_cert_der(&cert_pem)?;
    let key_der = pem_to_key_der(&key_pem)?;

    let config = ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(vec![cert_der], key_der)?;

    Ok(Arc::new(config))
}

fn generate_self_signed(local_ip: &str) -> anyhow::Result<(String, String)> {
    let mut params = CertificateParams::default();
    params.subject_alt_names = vec![
        SanType::IpAddress(local_ip.parse()?),
        SanType::DnsName("localhost".try_into()?),
        SanType::IpAddress("127.0.0.1".parse()?),
    ];
    // Valid for 10 years
    params.not_after = rcgen::date_time_ymd(2036, 1, 1);

    let key_pair = KeyPair::generate()?;
    let cert = params.self_signed(&key_pair)?;

    let cert_pem = cert.pem();
    let key_pem = key_pair.serialize_pem();

    Ok((cert_pem, key_pem))
}

fn pem_to_cert_der(pem: &str) -> anyhow::Result<CertificateDer<'static>> {
    // Strip PEM headers and decode base64
    let b64: String = pem
        .lines()
        .filter(|l| !l.starts_with("-----"))
        .collect::<Vec<&str>>()
        .join("");
    let der = base64_decode_simple(&b64)?;
    Ok(CertificateDer::from(der))
}

fn pem_to_key_der(pem: &str) -> anyhow::Result<PrivateKeyDer<'static>> {
    let b64: String = pem
        .lines()
        .filter(|l| !l.starts_with("-----"))
        .collect::<Vec<&str>>()
        .join("");
    let der = base64_decode_simple(&b64)?;
    Ok(PrivateKeyDer::Pkcs8(PrivatePkcs8KeyDer::from(der)))
}

fn base64_decode_simple(input: &str) -> anyhow::Result<Vec<u8>> {
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut table = [255u8; 256];
    for (i, &c) in CHARS.iter().enumerate() {
        table[c as usize] = i as u8;
    }

    let bytes: Vec<u8> = input
        .bytes()
        .filter(|b| *b != b'=' && *b != b'\n' && *b != b'\r' && *b != b' ')
        .collect();
    let mut result = Vec::with_capacity(bytes.len() * 3 / 4);

    for chunk in bytes.chunks(4) {
        let vals: Vec<u8> = chunk
            .iter()
            .filter_map(|&b| {
                let v = table[b as usize];
                if v == 255 { None } else { Some(v) }
            })
            .collect();
        if vals.len() >= 2 {
            result.push((vals[0] << 2) | (vals[1] >> 4));
        }
        if vals.len() >= 3 {
            result.push((vals[1] << 4) | (vals[2] >> 2));
        }
        if vals.len() >= 4 {
            result.push((vals[2] << 6) | vals[3]);
        }
    }
    Ok(result)
}
