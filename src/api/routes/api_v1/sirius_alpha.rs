use crate::api::routes::api_root::default::success;
use actix_web::{get, web, HttpResponse, Scope};
use actix_web::web::ServiceConfig;
use serde::{Deserialize, Serialize};
use crate::common::models::settings::Server;

#[derive(Debug, Deserialize, Serialize)]
pub struct SAlphaPingResponse {
    turn_buzzer_on: bool,
}

#[get("/ping")]
async fn ping() -> HttpResponse {
    let res = SAlphaPingResponse {
        turn_buzzer_on: false,
    };

    let de = serde_value::to_value(res).unwrap();

    success(de)
}

pub fn services(cfg: &mut web::ServiceConfig) {
    cfg.service(ping);
}


/// api sirius_alpha scope
/// path: {/api/v1}'/sirius_alpha'
pub fn sirius_alpha_scope(_: &Server) -> Scope {
    web::scope("/sirius_alpha").configure(sirius_alpha_services)
}

pub fn sirius_alpha_services(cfg: &mut ServiceConfig) {
    services(cfg);
}

