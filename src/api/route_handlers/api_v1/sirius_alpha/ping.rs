use crate::api::helpers::responses::success_resp;
use crate::AppState;
use actix_web::web::Json;
use actix_web::{http, put, web, HttpRequest, HttpResponse};
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
    app_state_data: web::Data<Mutex<AppState>>,
    body_data: Json<SAlphaPingRequest>,
    base_request: HttpRequest,
) -> anyhow::Result<HttpResponse, http::Error> {
    // todo add a custom error message result here. like server error instead of unwrap
    //todo fix unwrap
    let mut app_state_data_ok = app_state_data.lock().unwrap();
    //todo fix unwrap
    let device_id = base_request
        .head()
        .headers
        .get("x-device-id")
        .unwrap()
        .to_str()
        .unwrap();

    app_state_data_ok
        .iot_devices_state
        .insert_new(device_id, &body_data.device);

     println!("{:?}", app_state_data_ok);

    //todo :
    // session more previously started more than x minutes should be marked as a new session
    //

    let res = SAlphaPingResponse {
        short_period_buzzer_beep_duration_sec: 7,
        continuous_period_buzzer_beep_duration_sec: true,
    };

    Ok(success_resp(res))
}
