use serde::de::DeserializeOwned;
use std::fs::File;

use crate::common::errors::setup_errors::SetupError;
use crate::common::models::app_settings::AppSettings;
use crate::common::models::iot_settings::IotSettings;
use crate::constants::file_paths::FilePaths;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub app_settings: AppSettings,
    pub iot_settings: IotSettings,
}

impl AppConfig {
    fn settings_file<D>(file_path: &'static str) -> anyhow::Result<D>
    where
        D: DeserializeOwned,
    {
        log::debug!("reading the setting file: {}", file_path);

        let f = File::open(file_path);
        let f_ok = match f {
            Ok(f) => f,
            Err(e) => {
                return Err(SetupError::SettingsFile(file_path, e, "P00002").into());
            }
        };

        let data: Result<D, serde_yaml::Error> = serde_yaml::from_reader(f_ok);

        let out = match data {
            Ok(d) => d,
            Err(e) => {
                return Err(SetupError::SettingsFileDeserialize(file_path, e, "P00003").into());
            }
        };

        Ok(out)
    }

    pub fn new() -> anyhow::Result<Self> {
        log::debug!("reading the config files...");

        let app_settings = Self::settings_file(FilePaths::APP_SETTINGS)?;
        let iot_settings = Self::settings_file(FilePaths::IOT_SETTINGS)?;

        Ok(Self {
            app_settings,
            iot_settings,
        })
    }
}
