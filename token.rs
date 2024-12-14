use sha2::{Sha512, Digest};
use base64::{engine::general_purpose, Engine as _};
use std::time::{SystemTime, UNIX_EPOCH};

fn generate_signed_url(key: &str, url: &str, expiration: u64, client_ip: Option<&str>) -> String {
    let expires = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() + expiration;

    let mut base = format!("{}{}{}", key, url, expires);
    if let Some(ip) = client_ip {
        base.push_str(ip);
    }

    let mut hasher = Sha512::new();
    hasher.update(base.as_bytes());
    let hash = hasher.finalize();

    let mut token = general_purpose::URL_SAFE_NO_PAD.encode(hash);
    token = token.replace("+", "-").replace("/", "_");

    format!("{}?token={}&expires={}", url, token, expires)
}

fn main() {
    let key = "token_from_dashboard";
    let url = "your_url"; // e.g. https://spaceprotect.net/images/logo.png
    let expiration = 3600; // In seconds

    // This client IP address is ONLY required when IP Validation is enabled in the dashboard, otherwise remove this value
    let client_ip = Some("1.1.1.1");

    let signed_url = generate_signed_url(key, url, expiration, client_ip);
    println!("{}", signed_url);
}
