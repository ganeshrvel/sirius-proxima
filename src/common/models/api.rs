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
