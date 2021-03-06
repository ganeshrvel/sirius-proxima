pub mod helpers;
pub mod route_handlers;

use crate::api::route_handlers::{api_root_services, api_v1_scope};
use crate::common::models::app_settings::Server;
use crate::constants::header_keys::HeaderKeys;
use actix_web::{guard, web, Scope};

/// api scope
/// path: '/api'
pub fn api_scope(server: &Server) -> Scope {
    const ALLOWED_CONTENT_TYPE_KEY_VALUE: &str = "application/json";

    web::scope("/api")
        .service(
            api_v1_scope(server)
                .guard(guard::Any(guard::Get()).or(guard::Header(
                    HeaderKeys::CONTENT_TYPE,
                    ALLOWED_CONTENT_TYPE_KEY_VALUE,
                )))
                .guard(guard::Any(guard::Put()).or(guard::Header(
                    HeaderKeys::CONTENT_TYPE,
                    ALLOWED_CONTENT_TYPE_KEY_VALUE,
                )))
                .guard(guard::Any(guard::Options()).or(guard::Header(
                    HeaderKeys::CONTENT_TYPE,
                    ALLOWED_CONTENT_TYPE_KEY_VALUE,
                ))),
        )
        .configure(api_root_services)
}
