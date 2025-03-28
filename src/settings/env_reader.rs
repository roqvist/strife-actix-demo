use base64::prelude::*;
use dotenv::dotenv;
use reqwest::Identity;

/// # from_env()
/// Read required connection settings from environment (and .env) 
/// 
/// returns the tuple `(db_name, db_url, Identity)`
pub fn from_env() -> std::io::Result<(String, String, Identity)> {
    dotenv().ok();
    // Read everything from env
    let pkcs12_base64 = std::env::var("STRIFE_CERTIFICATE")
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidInput, "STRIFE_CERTIFICATE must be set." ))?;

    let cert_password = std::env::var("STRIFE_CERTIFICATE_PASSWORD")
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidInput, "STRIFE_CERTIFICATE_PASSWORD must be set."))?;

    let db_name = std::env::var("STRIFE_DATABASE")
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidInput, "STRIFE_DATABASE must be set."))?;

    let db_urls = std::env::var("STRIFE_DATABASE_URLS")
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidInput, "STRIFE_DATABASE_URLS must be set."))?;

    // Create required data
    let pkcs12_bytes = BASE64_STANDARD.decode(&pkcs12_base64)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidInput, format!("Base64 decoding STRIFE_CERTIFICATE failed: {}", e)))?;

    let identity = Identity::from_pkcs12_der(&pkcs12_bytes, &cert_password)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidInput, format!("Unable to create Identity from certificate: {}", e)))?;

    let db_url = db_urls
        .split(',')
        .next()
        .ok_or(std::io::Error::new(std::io::ErrorKind::InvalidInput, "STRIFE_DATABASE_URLS must be set."))?;

    Ok((db_name, db_url.into(), identity))
}
