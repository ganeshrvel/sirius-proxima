use crate::common::models::api::{ErrorResponse, SuccessResponse};
use crate::NotFoundResponse;
use actix_http::StatusCode;
use actix_web::{HttpResponse, HttpResponseBuilder};
use std::ops::Deref;

pub fn not_found() -> HttpResponse {
    HttpResponse::NotFound().json(NotFoundResponse {
        status_code: StatusCode::NOT_FOUND.as_u16(),
        message: StatusCode::NOT_FOUND.canonical_reason(),
        success: StatusCode::NOT_FOUND.is_success(),
    })
}

pub fn success_resp(data: impl serde::Serialize) -> HttpResponse {
    let data = serde_value::to_value(data);

    match data {
        Ok(d) => {
            let de = SuccessResponse {
                status_code: StatusCode::OK.as_u16(),
                message: StatusCode::OK.canonical_reason(),
                success: StatusCode::OK.is_success(),
                data: d,
            };

            HttpResponse::Ok().json(de)
        }
        Err(e) => error_resp(
            HttpResponse::InternalServerError(),
            StatusCode::INTERNAL_SERVER_ERROR,
            Some(e.to_string().deref()),
        ),
    }
}

pub fn error_resp(
    mut http_response: HttpResponseBuilder,
    status_code: StatusCode,
    err: Option<&str>,
) -> HttpResponse {
    http_response.json(error_resp_gen(status_code, err))
}

pub fn error_resp_gen(status_code: StatusCode, err: Option<&str>) -> ErrorResponse {
    ErrorResponse {
        status_code: status_code.as_u16(),
        message: status_code.canonical_reason(),
        success: status_code.is_success(),
        error: err,
    }
}
