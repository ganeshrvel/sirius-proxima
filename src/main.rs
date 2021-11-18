#[macro_use]
mod macros;

mod api;
mod common;
mod constants;
mod helpers;
mod utils;

use std::env;
use std::ops::Deref;

use actix_web::{middleware as actix_middleware, web, App, HttpServer};
use api::helpers::responses::not_found;

use crate::common::errors::setup_errors::SetupError;
use crate::common::models::api::NotFoundResponse;
use crate::common::models::data::AppData;
use crate::constants::app_env::AppEnv;
use crate::constants::strings::Strings;
use crate::helpers::actix::actix::{get_identity_service, get_json_err};
use crate::utils::logs::fern_log::setup_logging;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    println!("initializing the logger...");
    let s = setup_logging();

    if let Err(e) = s {
        return Err(SetupError::LoggerError(e, "P00001").into());
    }

    log::debug!("-----------------");
    log::debug!("Launching {}...", Strings::APP_NAME);

    if let Err(e) = run().await {
        log::error!("{:?}", e);

        return Err(e);
    }

    Ok(())
}

async fn run() -> anyhow::Result<()> {
    if AppEnv::IS_DEBUG {
        env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    }

    let data = AppData::new().await?;
    let data = actix_web::web::Data::new(data);

    log::info!(
        "starting the server on: {}",
        data.config.app_settings.settings.server.get_uri(true)
    );

    let data_cloned = data.clone();

    let h = HttpServer::new(move || {
        let dc = data_cloned.clone();

        App::new()
            .wrap(actix_middleware::Logger::default())
            .wrap(
                actix_middleware::DefaultHeaders::new()
                    .header("Permissions-Policy", "interest-cohort=()"),
            )
            .wrap(get_identity_service(
                dc.config.app_settings.settings.server.cookie_secret.deref(),
                dc.config.app_settings.settings.server.domain.deref(),
                dc.config.app_settings.settings.server.cookie_max_age_secs,
            ))
            .wrap(actix_middleware::Compress::default())
            .wrap(actix_middleware::NormalizePath::new(
                actix_middleware::TrailingSlash::Trim,
            ))
            .service(api::api_scope(&dc.config.app_settings.settings.server))
            .app_data(dc)
            .app_data(get_json_err())
            .default_service(web::to(not_found))
    })
    .bind(data.config.app_settings.settings.server.get_uri(false))?
    .run()
    .await?;

    Ok(h)
}
