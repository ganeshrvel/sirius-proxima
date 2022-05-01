use crate::common::errors::api_errors;
use crate::common::errors::api_errors::map_to_internal_server_error;
use crate::common::models::api::{ErrorResponse, SuccessResponse};
use crate::AppEnv;
use actix_http::StatusCode;
use actix_web::{HttpRequest, HttpResponse};

pub fn not_found() -> HttpResponse {
    HttpResponse::NotFound().json(ErrorResponse {
        status_code: StatusCode::NOT_FOUND.as_u16(),
        message: StatusCode::NOT_FOUND.canonical_reason(),
        success: StatusCode::NOT_FOUND.is_success(),
        error: None,
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
        Err(e) => http_error_resp(StatusCode::INTERNAL_SERVER_ERROR, Some(&*e.to_string())),
    }
}

pub fn http_error_resp(status_code: StatusCode, err: Option<&str>) -> HttpResponse {
    HttpResponse::build(status_code).json(http_error_resp_gen(status_code, err))
}

pub fn http_error_resp_gen(status_code: StatusCode, err: Option<&str>) -> ErrorResponse {
    let error = if AppEnv::IS_DEBUG { err } else { None };

    ErrorResponse {
        status_code: status_code.as_u16(),
        message: status_code.canonical_reason(),
        success: status_code.is_success(),
        error,
    }
}

pub fn get_http_header(
    base_request: &HttpRequest,
    key: &str,
) -> Result<String, api_errors::ApiErrors> {
    let header_value = base_request.head().headers.get(key);

    let header_value_ok = header_value.ok_or_else(|| {
        api_errors::ApiErrors::BadRequest(format!(
            r"Invalid request for the key in the header: {:?}",
            key
        ))
    })?;

    let value = header_value_ok
        .to_str()
        .map_err(map_to_internal_server_error)?;

    Ok(value.to_owned())
}
