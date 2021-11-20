use crate::api::route_handlers::api_v1::sirius_alpha::ping::salpha_ping;
use crate::common::models::settings::Server;
use actix_web::web::ServiceConfig;
use actix_web::{web, Scope};

mod ping;

// api sirius_alpha scope
// path: {/api/v1}'/sirius_alpha'
pub fn sirius_alpha_scope(_: &Server) -> Scope {
    web::scope("/sirius_alpha").configure(sirius_alpha_services)
}

pub fn sirius_alpha_services(cfg: &mut ServiceConfig) {
    cfg.service(salpha_ping);
}
