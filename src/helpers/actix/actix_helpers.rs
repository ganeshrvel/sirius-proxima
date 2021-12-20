use crate::common::models::app_settings::ServerTls;
use crate::HeaderKeys;
use actix_http::ServiceConfig;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::error::InternalError;
use actix_web::http::StatusCode;
use actix_web::web::JsonConfig;
use openssl::ssl::{SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod};

pub fn get_json_err() -> JsonConfig {
    JsonConfig::default()
        .error_handler(|err, _| InternalError::new(err, StatusCode::BAD_REQUEST).into())
}

pub fn get_identity_service(
    cookie_secret: &str,
    domain: &str,
    cookie_max_age_secs: i64,
    secure: bool,
) -> IdentityService<CookieIdentityPolicy> {
    IdentityService::new(
        CookieIdentityPolicy::new(cookie_secret.as_bytes())
            .name(HeaderKeys::AUTHORIZATION)
            .max_age_secs(cookie_max_age_secs)
            .domain(domain)
            .secure(secure),
    )
}

pub fn make_openssl_builder(t: &ServerTls) -> anyhow::Result<SslAcceptorBuilder> {
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls())?;
    builder.set_private_key_file(t.tls_key_file.clone(), SslFiletype::PEM)?;
    builder.set_certificate_chain_file(t.tls_cert_file.clone())?;

    Ok(builder)
}
