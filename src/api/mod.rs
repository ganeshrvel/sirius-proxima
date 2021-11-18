pub mod routes;

use crate::api::routes::{api_root_scope, api_v1_scope};
use crate::common::models::settings::Server;
use actix_web::{web, Scope};

pub fn api_scope(server: &Server) -> Scope {
    web::scope("/api")
        .service(api_v1_scope(server))
        .service(api_root_scope(server))
}
