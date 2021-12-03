use crate::api::helpers::responses::success_resp;
use crate::common::models::data::SharedAppData;
use actix_web::web::Json;
use actix_web::{http, put, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::common::models::iot_devices::{IotDevice, IotDeviceType};
use crate::common::models::iot_settings::{IotSettings, SAlphaIotPresets};
use crate::common::states::app_state::{
    IotDeviceAppState, IotDevicesActivityContainer, SAlphaAppState, SharedAppState,
};

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
        let device_states = &iot_device_activity.device_states;
        let salpha_app_state: &SAlphaAppState;
        match device_states {
            IotDeviceAppState::RoofWaterHeater(d)
            | IotDeviceAppState::BoreWellMotor(d)
            | IotDeviceAppState::GroundWellMotor(d) => {
                salpha_app_state = d;
            }
        }

        match device_type {
            IotDeviceType::RoofWaterHeater => Self::fetch_response_per_device(
                iot_device_activity,
                &iot_settings.settings.presets.roof_water_heater,
                salpha_app_state,
            ),
            IotDeviceType::BoreWellMotor => Self::fetch_response_per_device(
                iot_device_activity,
                &iot_settings.settings.presets.bore_well_motor,
                salpha_app_state,
            ),
            IotDeviceType::GroundWellMotor => Self::fetch_response_per_device(
                iot_device_activity,
                &iot_settings.settings.presets.ground_well_motor,
                salpha_app_state,
            ),
        }
    }

    // fetch the response for the `sirius alpha IOT` device
    fn fetch_response_per_device(
        iot_device_activity: &IotDevicesActivityContainer,
        salpha_presets: &SAlphaIotPresets,
        salpha_app_state: &SAlphaAppState,
    ) -> Self {
        let mut short_period_buzzer_beep_duration_ms: usize = 0;

        // if the [total_running_time] is zero (not started yet) then reset the [last_short_period_buzzer_activity_time] and [start_continuous_period_buzzer_beep_after_ms]
        if iot_device_activity.total_running_time.is_zero() {
            salpha_app_state.reset_continuous_period_buzzer_activity_time();
            salpha_app_state.reset_short_period_buzzer_activity_time();
        }
        // if the [total_running_time] of the iot device is not zero (the `sirius alpha IOT` device is active)
        else {
            let should_activate_short_period_buzzer: bool;
            let last_short_period_buzzer_activity_time_cell = salpha_app_state
                .last_short_period_buzzer_activity_time
                .get();

            // if the `sirius alpha IOT` device has already been activated (beeped) before, we use the time difference between the [last_activity_time] and [last_short_period_buzzer_activity_time] to check if the beep can be activated now again or not
            if let Some(last_short_period_buzzer_activity_time) =
                last_short_period_buzzer_activity_time_cell
            {
                // time difference between the [last_activity_time] and [last_short_period_buzzer_activity_time]
                let time_diff =
                    iot_device_activity.last_activity_time - last_short_period_buzzer_activity_time;

                // if the `time difference` is greater than the [interval_between_beeps_to_start_short_period_buzzer_ms] value in the salpha presets then we activate the buzzer
                should_activate_short_period_buzzer = time_diff.num_milliseconds()
                    >= salpha_presets.interval_between_beeps_to_start_short_period_buzzer_ms;
            }
            // if the `sirius alpha IOT` device has never been activated (beeped) before
            // and if the [total_running_time] is greater than the [interval_between_beeps_to_start_short_period_buzzer_ms] value in the salpha presets then we activate the buzzer
            else {
                should_activate_short_period_buzzer =
                    iot_device_activity.total_running_time.num_milliseconds()
                        >= salpha_presets.interval_between_beeps_to_start_short_period_buzzer_ms;
            }

            // if [should_activate_short_period_buzzer] is true then we start the shor period buzzer to beep [short_period_buzzer_beep_duration_ms] duration.
            // else if [should_activate_short_period_buzzer] is false then we do not update the [last_short_period_buzzer_activity_time] and in the next api request cycle the [should_activate_short_period_buzzer] will be false if the time difference isnt met
            if should_activate_short_period_buzzer {
                short_period_buzzer_beep_duration_ms =
                    salpha_presets.short_period_buzzer_beep_duration_ms;

                // update the [last_short_period_buzzer_activity_time] to now();
                salpha_app_state.update_short_period_buzzer_activity_time();
            }
        }

        // should or not to activate the continuous period buzzer
        let should_continuous_period_buzzer_beep =
            iot_device_activity.total_running_time.num_milliseconds()
                >= salpha_presets.start_continuous_period_buzzer_beep_after_ms;

        if should_continuous_period_buzzer_beep {
            // update continuous period buzzer activity time to now();
            salpha_app_state.update_continuous_period_buzzer_activity_time();
        }

        Self {
            short_period_buzzer_beep_duration_ms,
            is_continuous_period_buzzer_beep_active: should_continuous_period_buzzer_beep,
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

    //  println!("{:?}", shared_app_data.config.iot_settings.settings.presets.bore_well_motor.);

    //todo :
    // session more previously started more than x minutes should be marked as a new session
    //

    let res = SAlphaPingResponse::new(iot_device_activity, device_type, iot_settings);

   // println!("{:?}", app_state_data_ok);

    /*{
        short_period_buzzer_beep_duration_ms: 7,
        continuous_period_buzzer_beep_duration_ms: true,
    };*/

    Ok(success_resp(res))
}
