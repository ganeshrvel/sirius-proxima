use crate::helpers::parsers::setting_files::AppConfig;
use std::sync::Arc;
use actix_web::web;

pub type SharedAppData = web::Data<Arc<AppData>>;

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
