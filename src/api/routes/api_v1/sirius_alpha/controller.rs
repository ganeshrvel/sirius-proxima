use crate::api::helpers::responses::success_resp;
use actix_web::web::Json;
use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub enum SAlphaDevice {
    #[serde(rename = "water_heater")]
    WaterHeater,

    #[serde(rename = "bore_well")]
    BoreWell,

    #[serde(rename = "ground_well")]
    GroundWell,
}

#[derive(Debug, serde::Deserialize)]
pub struct SAlphaPingRequest {
    pub device: SAlphaDevice,
    pub device_name: String,
    pub app_version: u8,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SAlphaPingResponse {
    pub turn_buzzer_on: bool,
}

pub fn salpha_ping(req: Json<SAlphaPingRequest>) -> HttpResponse {
    let res = SAlphaPingResponse {
        turn_buzzer_on: false,
    };

    success_resp(res)
}
