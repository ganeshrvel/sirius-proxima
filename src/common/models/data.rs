use crate::helpers::parsers::setting_files::AppConfig;
use std::sync::Arc;

pub struct AppData {
    pub config: AppConfig,
}

impl AppData {
    pub async fn new() -> anyhow::Result<Arc<AppData>> {
        let c = AppConfig::new()?;

        Ok(Arc::new(Self { config: c }))
    }
}
