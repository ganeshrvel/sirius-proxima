use crate::helpers::parsers::setting_files::AppConfig;
use std::sync::Arc;

pub struct AppData<'a> {
    pub config: AppConfig<'a>,
}

impl<'a> AppData<'a> {
    pub async fn new() -> anyhow::Result<Arc<AppData<'a>>> {
        let c = AppConfig::new()?;

        Ok(Arc::new(Self { config: c }))
    }
}
