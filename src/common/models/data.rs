use crate::common::errors::setup_errors::SetupError;
use crate::helpers::parsers::setting_files::AppConfig;
use std::sync::Arc;

pub(crate) struct AppData {
    pub config: AppConfig,
}

impl AppData {
    pub async fn new() -> anyhow::Result<Arc<Self>> {
        let config = AppConfig::new()?;

        Ok(Arc::new(Self { config }))
    }
}
