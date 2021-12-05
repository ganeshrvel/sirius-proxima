use crate::api::helpers::responses::http_error_resp;
use crate::common::errors::api_errors;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use std::fmt::{Display, Formatter};
use std::sync::{MutexGuard, TryLockError};

#[derive(Debug)]
pub enum ApiErrors {
    MutexGuard(String),

    Http(StatusCode, String),

    InternalServerError(String),

    BadRequest(String),
}

impl ApiErrors {
    pub fn message(&self) -> String {
        match self {
            Self::MutexGuard(e) => format!(r"A Mutex Guard error occured: {:?}", e),
            Self::InternalServerError(e) => format!(r"An Internal Server Error occured: {:?}", e),
            Self::BadRequest(e) => format!(r"A Bad Request Error occured: {:?}", e),
            Self::Http(status_code, e) => {
                format!(r"A HTTP error occured: {:?} | {:?}", status_code, e)
            }
        }
    }
}

impl Display for ApiErrors {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ResponseError for ApiErrors {
    fn error_response(&self) -> HttpResponse {
        let message = self.message();

        match self {
            ApiErrors::MutexGuard(e) => {
                log::error!(r"[ErrorResponse] [MutexGuardError]: {:?}", message,);

                http_error_resp(StatusCode::INTERNAL_SERVER_ERROR, Option::Some(&*message))
            }
            ApiErrors::InternalServerError(e) => {
                log::error!(r"[ErrorResponse] [InternalServerError]: {:?}", message,);

                http_error_resp(StatusCode::INTERNAL_SERVER_ERROR, Option::Some(&*message))
            }
            ApiErrors::BadRequest(e) => {
                log::error!(r"[ErrorResponse] [BadRequest]: {:?}", message,);

                http_error_resp(StatusCode::BAD_REQUEST, Option::Some(&*message))
            }
            ApiErrors::Http(status_code, e) => {
                log::error!(r"[ErrorResponse] [HttpError]: {:?}", message,);

                http_error_resp(*status_code, Option::Some(&*message))
            }
        }
    }
}

pub fn map_mutex_guard_to_api_error<T>(e: TryLockError<MutexGuard<T>>) -> ApiErrors {
    api_errors::ApiErrors::MutexGuard(e.to_string())
}

pub fn map_to_internal_server_error<E>(e: E) -> ApiErrors
where
    E: Display,
{
    api_errors::ApiErrors::InternalServerError(e.to_string())
}
