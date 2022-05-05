use local_ip_address::local_ip;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub settings: SettingsEntity,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SettingsEntity {
    pub server: Server,
    pub telegram: TelegramEntity,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Server {
    pub cookie_secret: String,

    pub cookie_max_age_secs: i64,

    pub api_secret_key: String,

    pub api_secret_token: String,

    pub ip: Option<String>,

    pub port: u32,

    pub domain: Option<String>,

    pub enable_tls: bool,

    pub tls: Option<ServerTls>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TelegramEntity {
    pub token: String,
    pub chat_id: i64,
}

impl Server {
    pub fn get_domain(&self) -> anyhow::Result<String> {
        if let Some(domain) = &self.domain {
            return Ok(domain.clone());
        }

        let resolved_domain = self.get_uri(true)?;

        Ok(resolved_domain)
    }

    pub fn get_ip(&self) -> anyhow::Result<String> {
        if let Some(ip) = &self.ip {
            return Ok(ip.clone());
        }

        let the_local_ip = local_ip()?;

        Ok(the_local_ip.to_string())
    }

    pub fn get_uri(&self, prefix_protocol: bool) -> anyhow::Result<String> {
        let ip = self.get_ip()?;

        if !prefix_protocol {
            return Ok(format!("{}:{}", ip, self.port));
        }

        Ok(format!(
            "{}://{}:{}",
            if self.enable_tls { "https" } else { "http" },
            ip,
            self.port
        ))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ServerTls {
    pub tls_key_file: String,

    pub tls_cert_file: String,
}
