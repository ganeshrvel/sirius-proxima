pub mod helpers;
pub mod route_handlers;

use crate::api::route_handlers::{api_root_services, api_v1_scope};
use crate::common::models::app_settings::Server;
use actix_web::{guard, web, Scope};

/// api scope
/// path: '/api'
pub fn api_scope(server: &Server) -> Scope {
    web::scope("/api")
        .service(
            api_v1_scope(server)
                .guard(
                    guard::Any(guard::Get()).or(guard::Header("Content-Type", "application/json")),
                )
                .guard(
                    guard::Any(guard::Put()).or(guard::Header("Content-Type", "application/json")),
                )
                .guard(
                    guard::Any(guard::Options())
                        .or(guard::Header("Content-Type", "application/json")),
                ),
        )
        .configure(api_root_services)
}
