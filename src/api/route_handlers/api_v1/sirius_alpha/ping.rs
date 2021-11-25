use crate::api::helpers::responses::success_resp;
use crate::AppState;
use actix_web::web::Json;
use actix_web::{http, put, web, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::common::models::data::{IotDevice, IotDeviceType};
use std::sync::Mutex;

#[derive(Debug, serde::Deserialize, Clone)]
pub struct SAlphaPingRequest {
    pub device: IotDevice,
    pub device_type: IotDeviceType,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SAlphaPingResponse {
    pub short_period_buzzer_beep_duration_sec: u16,
    pub continuous_period_buzzer_beep_duration_sec: bool,
}

#[put("/ping")]
#[allow(clippy::unused_async)]
pub async fn salpha_ping(
    data: web::Data<Mutex<AppState>>,
    req: Json<SAlphaPingRequest>,
) -> anyhow::Result<HttpResponse, http::Error> {
    //todo fix this
    let mut data = data.lock().unwrap();

    data.iot_devices_state
        .insert_new(req.device_type, &req.device);

   // println!("{:?}", data);

    let res = SAlphaPingResponse {
        short_period_buzzer_beep_duration_sec: 7,
        continuous_period_buzzer_beep_duration_sec: true,
    };

    Ok(success_resp(res))
}
