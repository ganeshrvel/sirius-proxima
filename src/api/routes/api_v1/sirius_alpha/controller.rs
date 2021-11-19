use crate::api::helpers::responses::success_resp;
use actix_web::web::Json;
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Debug, Deserialize)]
pub struct SAlphaDeviceDetails {
    pub device_name: String,
    pub model: String,
    pub device_id: String,
    pub device_location: String,
    pub device_sdk: String,
    pub app_version: String,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "device_type", content = "details")]
pub enum SAlphaDevice {
    #[serde(rename = "water_heater")]
    WaterHeater(SAlphaDeviceDetails),

    #[serde(rename = "bore_well")]
    BoreWell(SAlphaDeviceDetails),

    #[serde(rename = "ground_well")]
    GroundWell(SAlphaDeviceDetails),
}

#[derive(Debug, serde::Deserialize)]
pub struct SAlphaPingRequest {
    pub device: SAlphaDevice,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SAlphaPingResponse {
    pub short_period_buzzer_beep_duration_sec: u16,
    pub continuous_period_buzzer_beep_duration_sec: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AppState {
    pub counter: usize,
}

pub fn salpha_ping(data: web::Data<Mutex<AppState>>, req: Json<SAlphaPingRequest>) -> HttpResponse {
    let mut data = data.lock().unwrap();
    data.counter += 1;
    println!("{}", data.counter);

    let res = SAlphaPingResponse {
        short_period_buzzer_beep_duration_sec: 7,
        continuous_period_buzzer_beep_duration_sec: true,
    };

    success_resp(res)
}
