use crate::api::helpers::responses::success_resp;
use crate::common::models::data::SharedAppData;
use actix_web::web::Json;
use actix_web::{http, put, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::common::models::iot_devices::{IotDevice, IotDeviceType};
use crate::common::models::iot_settings::{IotSettings, SAlphaIotPresets};
use crate::common::states::app_state::{IotDevicesActivityContainer, SharedAppState};

#[derive(Debug, Deserialize, Clone)]
pub struct SAlphaPingRequest {
    pub device: IotDevice,
    pub device_type: IotDeviceType,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SAlphaPingResponse {
    pub short_period_buzzer_beep_duration_ms: usize,
    pub is_continuous_period_buzzer_beep_active: bool,
}

impl SAlphaPingResponse {
    fn new(
        iot_device_activity: &IotDevicesActivityContainer,
        device_type: IotDeviceType,
        iot_settings: &IotSettings,
    ) -> Self {
        Self::fetch_response(iot_device_activity, device_type, iot_settings)
    }

    fn fetch_response(
        iot_device_activity: &IotDevicesActivityContainer,
        device_type: IotDeviceType,
        iot_settings: &IotSettings,
    ) -> Self {
        match device_type {
            IotDeviceType::RoofWaterHeater => Self::fetch_response_per_device(
                iot_device_activity,
                &iot_settings.settings.presets.roof_water_heater,
            ),
            IotDeviceType::BoreWellMotor => Self::fetch_response_per_device(
                iot_device_activity,
                &iot_settings.settings.presets.bore_well_motor,
            ),
            IotDeviceType::GroundWellMotor => Self::fetch_response_per_device(
                iot_device_activity,
                &iot_settings.settings.presets.ground_well_motor,
            ),
        }
    }

    fn fetch_response_per_device(
        iot_device_activity: &IotDevicesActivityContainer,
        salpha_presets: &SAlphaIotPresets,
    ) -> Self {
        let is_continuous_period_buzzer_beep_active =
            iot_device_activity.total_running_time.num_milliseconds()
                >= salpha_presets.start_continuous_period_buzzer_beep_after_ms;

        let mut short_period_buzzer_beep_duration_ms: usize = 0;
        if !iot_device_activity.total_running_time.is_zero() {
            // todo after the session reset, the short_period_buzzer_beep_duration_ms goes back to 7000 in 2 attempts.
            //      the time_diff should be between now and the last beep
            // todo keep a state session for [short_period_buzzer_beep_duration_ms]
            if let Some(activity) = iot_device_activity.data_storage.first() {
                let time_diff = iot_device_activity.last_activity_time - activity.time;

                if time_diff.num_milliseconds()
                    >= salpha_presets.interval_between_beeps_to_start_short_period_buzzer_ms
                {
                    short_period_buzzer_beep_duration_ms =
                        salpha_presets.short_period_buzzer_beep_duration_ms;
                }
            }
        }

        Self {
            short_period_buzzer_beep_duration_ms,
            is_continuous_period_buzzer_beep_active,
        }
    }
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

    let device_type = body_data.device_type;
    let iot_device = &body_data.device;
    let iot_settings = &shared_app_data.config.iot_settings;

    app_state_data_ok.iot_devices_state.insert_new(
        device_id,
        device_type,
        iot_device,
        &shared_app_data,
    );

    //todo fix unwrap
    let iot_device_activity = app_state_data_ok
        .iot_devices_state
        .devices_activity_bucket
        .get(device_id)
        .unwrap();

    //println!("{:?}", app_state_data_ok);
    //  println!("{:?}", shared_app_data.config.iot_settings.settings.presets.bore_well_motor.);

    //todo :
    // session more previously started more than x minutes should be marked as a new session
    //

    let res = SAlphaPingResponse::new(iot_device_activity, device_type, iot_settings);

    /*{
        short_period_buzzer_beep_duration_ms: 7,
        continuous_period_buzzer_beep_duration_ms: true,
    };*/

    Ok(success_resp(res))
}
