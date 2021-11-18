use crate::api::routes::api_v1::sirius_alpha::controller::{salpha_ping, SAlphaPingRequest};
use actix_web::web::Json;
use actix_web::{put, web, HttpResponse};

#[put("/ping")]
pub async fn ping(req: Json<SAlphaPingRequest>) -> HttpResponse {
    salpha_ping(req)
}

pub fn services(cfg: &mut web::ServiceConfig) {
    cfg.service(ping);
}
