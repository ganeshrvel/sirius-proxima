use crate::api::route_handlers::api_root::unprotected;
use crate::api::route_handlers::api_v1::sirius_alpha::sirius_alpha_scope;
use crate::common::models::app_settings::Server;
use actix_web::web::ServiceConfig;
use actix_web::{guard, web, Scope};

pub mod api_root;
pub mod api_v1;

/// api v1 scope
/// path: {/api}'/v1'
pub fn api_v1_scope(server: &Server) -> Scope {
    let api_secret_token = server.api_secret_token.clone();
    let api_secret_key = server.api_secret_key.clone();

    web::scope("/v1")
        .guard(guard::fn_guard(move |head| {
            let header_value = head.headers.get(&api_secret_key);

            match header_value {
                None => false,
                Some(d) => d.eq(&api_secret_token),
            }
        }))
        .guard(guard::fn_guard(move |head| {
            let header_value = head.headers.get("X-Device-id");

            match header_value {
                None => false,
                Some(d) => !d.is_empty(),
            }
        }))
        .service(sirius_alpha_scope(server))
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

/// api root scope
/// path: {/api}'/'
pub fn api_root_services(cfg: &mut ServiceConfig) {
    unprotected::health::services(cfg);
}
