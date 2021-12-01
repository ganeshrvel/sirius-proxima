use crate::helpers::parsers::setting_files::AppConfig;
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
