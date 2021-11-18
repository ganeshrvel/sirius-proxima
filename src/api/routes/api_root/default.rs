use crate::common::models::api::SuccessResponse;
use crate::ErrorResponse;
use actix_http::StatusCode;
use actix_web::HttpResponse;

pub fn not_found() -> HttpResponse {
    HttpResponse::NotFound().json(ErrorResponse {
        status_code: StatusCode::NOT_FOUND.as_u16(),
        message: StatusCode::NOT_FOUND.canonical_reason(),
    })
}

pub fn success(data: serde_value::Value) -> HttpResponse {
    HttpResponse::Ok().json(SuccessResponse {
        status_code: StatusCode::OK.as_u16(),
        message: StatusCode::OK.canonical_reason(),
        data,
    })
}
