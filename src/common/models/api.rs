use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Health {
    pub success: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorResponse<'a> {
    pub status_code: u16,
    pub message: Option<&'a str>,
}

#[derive(Debug, Serialize)]
pub struct SuccessResponse<'a> {
    pub status_code: u16,
    pub message: Option<&'a str>,
    pub data: serde_value::Value,
}
