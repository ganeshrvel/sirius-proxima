pub mod helpers;
pub mod routes;

use crate::api::routes::{api_root_services, api_v1_scope};
use crate::common::models::settings::Server;
use actix_web::{web, Scope};

/// api scope
/// path: '/api'
pub fn api_scope(server: &Server) -> Scope {
    web::scope("/api")
        .service(api_v1_scope(server))
        .configure(api_root_services)
}
