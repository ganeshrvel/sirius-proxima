use std::ops::Deref;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::error::InternalError;
use actix_web::http::StatusCode;
use actix_web::web::JsonConfig;

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
            .name("Authorization")
            .max_age_secs(cookie_max_age_secs)
            .domain(domain.deref())
            .secure(false),
    )
}
