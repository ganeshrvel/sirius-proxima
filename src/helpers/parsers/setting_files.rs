use std::fs::File;

use crate::common::errors::setup_errors::SetupError;
use crate::common::models::settings::Settings;
use crate::constants::file_paths::FilePaths;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub app_settings: Settings,
}

impl AppConfig {
    fn settings_file() -> anyhow::Result<Settings> {
        let file_path = FilePaths::SETTINGS;
        log::debug!("reading the setting file... {}", file_path);

        let f = File::open(file_path);
        let f_ok = match f {
            Ok(f) => f,
            Err(e) => {
                return Err(SetupError::SettingsFile(file_path, e, "P00002").into());
            }
        };

        let data: Result<Settings, serde_yaml::Error> = serde_yaml::from_reader(f_ok);

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

        let s = Self::settings_file();

        Ok(AppConfig { app_settings: s? })
    }
}
