use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Eq, PartialEq, Hash)]
#[serde(tag = "device_type", content = "details")]
pub enum IotDevice {
    // esp-8266 01 device
    #[serde(rename = "roof_water_heater")]
    RoofWaterHeater(SAlphaDeviceDetails),

    // esp-8266 01 device
    #[serde(rename = "bore_well_motor")]
    BoreWellMotor(SAlphaDeviceDetails),

    // esp-8266 01 device
    #[serde(rename = "ground_well_motor")]
    GroundWellMotor(SAlphaDeviceDetails),
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash, Copy)]
pub enum IotDeviceType {
    #[serde(rename = "roof_water_heater")]
    RoofWaterHeater,

    #[serde(rename = "bore_well_motor")]
    BoreWellMotor,

    #[serde(rename = "ground_well_motor")]
    GroundWellMotor,
}

// SAlpha or sirius-alpha are the esp-8266 01 devices
#[derive(Debug, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct SAlphaDeviceDetails {
    pub device_name: String,
    pub model: String,
    pub device_id: String,
    pub device_location: String,
    pub device_sdk: String,
    pub app_version: String,
}
