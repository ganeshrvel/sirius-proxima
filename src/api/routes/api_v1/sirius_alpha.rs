use actix_web::{get, web, HttpResponse, Responder};

#[get("/build_details")]
async fn build_details() -> impl Responder {
    HttpResponse::Ok()
}

pub fn services(cfg: &mut web::ServiceConfig) {
    cfg.service(build_details);
}
