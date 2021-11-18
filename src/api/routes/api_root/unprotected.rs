use crate::common::models::api::Health;
use actix_web::{get, web, HttpRequest, HttpResponse, Responder};

#[get("/health")]
pub async fn health(req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().json(Health { success: true })
}

pub fn services(cfg: &mut web::ServiceConfig) {
    cfg.service(health);
}
