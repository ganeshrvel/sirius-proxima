use crate::api::helpers::responses::success_resp;
use crate::common::models::api::Health;
use actix_web::{get, web, HttpRequest, Responder};

#[get("/health")]
pub async fn health(_: HttpRequest) -> impl Responder {
    let res = Health { is_health_ok: true };

    success_resp(res)
}

pub fn services(cfg: &mut web::ServiceConfig) {
    cfg.service(health);
}
