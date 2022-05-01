use crate::common::models::iot_devices::IotDeviceType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IotSettings {
    pub settings: SettingsEntity,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SettingsEntity {
    pub presets: IotPresets,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IotPresets {
    pub roof_water_heater: SAlphaIotPresets,
    pub bore_well_motor: SAlphaIotPresets,
    pub ground_well_motor: SAlphaIotPresets,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SAlphaIotPresets {
    pub device_type: IotDeviceType,

    // the interval of time required between beeps to activate the short period buzzer
    pub interval_between_beeps_to_start_short_period_buzzer_ms: i64,

    // the total runtime after which the continuous buzzer should start
    pub start_continuous_period_buzzer_beep_after_ms: i64,

    // the duration of the short term buzzer beep
    pub short_period_buzzer_beep_duration_ms: usize,

    // the maximum intervals allowed between 2 activities of an IOT device to tag it as a single session
    // this is useful in judging whether the IOT is still in session in case of a power failure or other unknown reasons.
    pub max_interval_to_persist_session_ms: i64,

    // pause the device activity when there is no ping for [pause_total_running_time_on_inactive_for_ms]
    // this is done to take care of power outage while using the device and to pause the device activity when there is no ping for a preset amount of time
    // resume the device activity once the ping is received again
    // Note: if the activity is received after [max_interval_to_persist_session_ms] time then it will be considered as a new session
    pub pause_total_running_time_on_inactive_for_ms: i64,
}
