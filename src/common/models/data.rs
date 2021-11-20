use crate::helpers::parsers::setting_files::AppConfig;
use serde::Deserialize;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct AppData {
    pub config: AppConfig,
}

impl AppData {
    pub async fn new() -> anyhow::Result<Arc<Self>> {
        let c = AppConfig::new()?;

        Ok(Arc::new(Self { config: c }))
    }
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq, Hash)]
#[serde(tag = "device_type", content = "details")]
pub enum IotDevice {
    #[serde(rename = "water_heater")]
    WaterHeater(SAlphaDeviceDetails),

    #[serde(rename = "bore_well")]
    BoreWell(SAlphaDeviceDetails),

    #[serde(rename = "ground_well")]
    GroundWell(SAlphaDeviceDetails),
}

#[derive(Debug, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct SAlphaDeviceDetails {
    pub device_name: String,
    pub model: String,
    pub device_id: String,
    pub device_location: String,
    pub device_sdk: String,
    pub app_version: String,
}
