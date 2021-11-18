use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings<'a> {
    pub settings: SettingsEntity<'a>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SettingsEntity<'a> {
    pub server: Server<'a>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Server<'a> {
    pub port: u32,

    pub domain: Cow<'a, str>,

    pub cookie_secret: Cow<'a, str>,

    pub cookie_max_age_secs: i64,

    pub api_secret_token: &'a str,

    pub ip: Cow<'a, str>,
    pub https: bool,
}

impl<'a> Server<'a> {
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
