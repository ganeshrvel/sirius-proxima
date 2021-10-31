use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Settings {
    pub settings: SettingsEntity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct SettingsEntity {
    pub server: Server,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Server {
    pub port: u32,
    pub domain: String,
    pub cookie_secret: String,
    pub ip: String,
    pub https: bool,
}

impl Server {
    pub fn get_ip(&self) -> String {
        format!("{}:{}", self.ip, self.port)
    }
}
