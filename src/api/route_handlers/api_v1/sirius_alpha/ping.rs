use crate::api::helpers::responses::success_resp;
use crate::common::models::data::SharedAppData;
use crate::{AppData, AppState};
use actix_web::web::Json;
use actix_web::{http, put, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::common::models::iot_devices::{IotDevice, IotDeviceType};
use crate::common::states::app_state::SharedAppState;

#[derive(Debug, serde::Deserialize, Clone)]
pub struct SAlphaPingRequest {
    pub device: IotDevice,
    pub device_type: IotDeviceType,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SAlphaPingResponse {
    pub short_period_buzzer_beep_duration_ms: usize,
    pub continuous_period_buzzer_beep_duration_ms: bool,
}

#[put("/ping")]
#[allow(clippy::unused_async)]
pub async fn salpha_ping(
    shared_app_state: SharedAppState,
    shared_app_data: SharedAppData,
    body_data: Json<SAlphaPingRequest>,
    base_request: HttpRequest,
) -> anyhow::Result<HttpResponse, http::Error> {
    // todo add a custom error message result here. like server error instead of unwrap
    //todo fix unwrap
    let mut app_state_data_ok = shared_app_state.lock().unwrap();

    //todo fix unwrap
    //let app_data_ok = app_data.lock().unwrap();

    //todo fix unwrap
    let device_id = base_request
        .head()
        .headers
        .get("x-device-id")
        .unwrap()
        .to_str()
        .unwrap();

    app_state_data_ok.iot_devices_state.insert_new(
        device_id,
        body_data.device_type,
        &body_data.device,
        &shared_app_data,
    );

    //println!("{:?}", app_state_data_ok);
    //  println!("{:?}", shared_app_data.config.iot_settings.settings.presets.bore_well_motor.);

    //todo :
    // session more previously started more than x minutes should be marked as a new session
    //

    let res = SAlphaPingResponse {
        short_period_buzzer_beep_duration_ms: 7,
        continuous_period_buzzer_beep_duration_ms: true,
    };

    Ok(success_resp(res))
}
