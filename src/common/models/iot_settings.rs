use serde::{Deserialize, Serialize};
use crate::common::models::iot_devices::IotDeviceType;

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

    // the interval between each short term buzzer activity
    pub short_period_buzzer_interval_between_beep_ms: usize,

    // the interval of activity after which the continuous alert buzzer should start.
    pub start_continuous_period_buzzer_beep_after_ms: usize,

    // the duration of the short term buzzer beep
    pub short_period_buzzer_beep_duration_ms: usize,

    // the maximum intervals allowed between iot activities to tag it as a single session
    // this is useful in judging whether the iot is still in session in case of a power failure or other unknown reasons.
    pub max_interval_to_persist_session_ms: i64,
}
