use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub settings: SettingsEntity,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SettingsEntity {
    pub server: Server,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Server {
    pub port: u32,

    pub domain: String,

    pub cookie_secret: String,

    pub cookie_max_age_secs: i64,

    pub api_secret_token: String,

    pub ip: String,
    pub https: bool,
}

impl Server {
    pub fn get_uri(&self, prefix_protocol: bool) -> String {
        if !prefix_protocol {
            return format!("{}:{}", self.ip, self.port);
        }

        format!(
            "{}://{}:{}",
            if self.https { "https" } else { "http" },
            self.ip,
            self.port
        )
    }
}
