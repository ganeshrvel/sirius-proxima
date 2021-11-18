use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Health {
    pub is_health_ok: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NotFoundResponse<'a> {
    pub status_code: u16,
    pub message: Option<&'a str>,
    pub success: bool,
}

#[derive(Debug, Serialize)]
pub struct SuccessResponse<'a> {
    pub status_code: u16,
    pub message: Option<&'a str>,
    pub data: serde_value::Value,
    pub success: bool,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse<'a> {
    pub status_code: u16,
    pub message: Option<&'a str>,
    pub success: bool,
    pub error: Option<&'a str>,
}
