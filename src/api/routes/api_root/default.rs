use crate::ErrorResponse;
use actix_http::StatusCode;
use actix_web::HttpResponse;

pub fn not_found() -> HttpResponse {
    HttpResponse::NotFound().json(ErrorResponse {
        status_code: StatusCode::NOT_FOUND.as_u16(),
        message: StatusCode::NOT_FOUND.canonical_reason(),
    })
}
