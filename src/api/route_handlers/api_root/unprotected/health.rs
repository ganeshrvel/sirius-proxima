use crate::api::helpers::responses::success_resp;
use crate::common::models::api::Health;
use actix_web::{get, web, Error, HttpResponse};

#[get("/health")]
async fn health() -> anyhow::Result<HttpResponse, Error> {
    let res = Health { is_health_ok: true };

    Ok(success_resp(res))
}

pub fn services(cfg: &mut web::ServiceConfig) {
    cfg.service(health);
}
