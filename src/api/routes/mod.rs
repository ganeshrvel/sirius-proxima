use crate::api::routes::api_root::unprotected;
use crate::common::models::settings::Server;
use actix_web::web::ServiceConfig;
use actix_web::{guard, web, Scope};
use crate::api::routes::api_v1::sirius_alpha;

pub mod api_root;
pub mod api_v1;

/// api v1 scope
/// path: {/api}'/v1'
pub fn api_v1_scope(server: &Server) -> Scope {
    let api_secret_token = server.api_secret_token.clone();

    web::scope("/v1")
        .guard(guard::fn_guard(move |req| {
            match req.headers().get("API_TOKEN") {
                Some(value) => value == api_secret_token.as_str(),
                None => false,
            }
        }))
        .configure(api_v1_services)
}

pub fn api_v1_services(cfg: &mut ServiceConfig) {
    sirius_alpha::services(cfg)
}

/// api root scope
/// path: {/api}'/'
pub fn api_root_scope(_: &Server) -> Scope {
    web::scope("").configure(api_root_services)
}

pub fn api_root_services(cfg: &mut ServiceConfig) {
    unprotected::services(cfg);
}
