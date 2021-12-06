use crate::common::models::app_settings::ServerTls;
use crate::HeaderKeys;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::error::InternalError;
use actix_web::http::StatusCode;
use actix_web::web::JsonConfig;
use rustls::server::NoClientAuth;
use rustls::ServerConfig;
use std::{fs, io};

pub fn get_json_err() -> JsonConfig {
    JsonConfig::default()
        .error_handler(|err, _| InternalError::new(err, StatusCode::BAD_REQUEST).into())
}

pub fn get_identity_service(
    cookie_secret: &str,
    domain: &str,
    cookie_max_age_secs: i64,
) -> IdentityService<CookieIdentityPolicy> {
    IdentityService::new(
        CookieIdentityPolicy::new(cookie_secret.as_bytes())
            .name(HeaderKeys::AUTHORIZATION)
            .max_age_secs(cookie_max_age_secs)
            .domain(domain)
            .secure(false),
    )
}

pub fn make_server_config(t: &ServerTls) -> anyhow::Result<ServerConfig> {
    let client_auth = NoClientAuth::new();
    let cert_chain = rustls_pemfile::certs(&mut io::BufReader::new(fs::File::open(
        t.tls_cert_file.clone(),
    )?))?
    .iter()
    .map(|v| rustls::Certificate(v.clone()))
    .collect();

    let key_reader = &mut io::BufReader::new(fs::File::open(t.tls_key_file.clone())?);
    let key_buf = &rustls_pemfile::pkcs8_private_keys(key_reader)?[0];

    let key_der = rustls::PrivateKey((*key_buf).clone());

    let cfg = ServerConfig::builder()
        .with_safe_default_cipher_suites()
        .with_safe_default_kx_groups()
        .with_protocol_versions(&[&rustls::version::TLS13])?
        .with_client_cert_verifier(client_auth)
        .with_single_cert(cert_chain, key_der)?;

    Ok(cfg)
}
