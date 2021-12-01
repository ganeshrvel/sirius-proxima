#![deny(clippy::all)]
#![warn(
    clippy::all,
    clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    clippy::print_stdout
)]
#![allow(
    clippy::missing_docs_in_private_items,
    clippy::future_not_send,
    clippy::implicit_return,
    clippy::similar_names,
    clippy::blanket_clippy_restriction_lints,
    clippy::module_name_repetitions
)]


#[macro_use]
mod macros;

mod api;
mod common;
mod constants;
mod helpers;
mod utils;

use std::env;

use actix_web::{middleware as actix_middleware, web, App, HttpServer};
use api::helpers::responses::not_found;
use std::sync::Mutex;

use crate::common::errors::setup_errors::SetupError;
use crate::common::models::api::NotFoundResponse;
use crate::common::models::data::AppData;
use crate::common::states::app_state::AppState;
use crate::constants::app_env::AppEnv;
use crate::constants::default_values::DefaultValues;
use crate::constants::strings::Strings;
use crate::helpers::actix::actix_helpers::{get_identity_service, get_json_err};
use crate::helpers::sanitizers::sanitize_constants;
use crate::utils::logs::fern_log::setup_logging;
use crate::utils::vectors::push_to_last_and_maintain_capacity_of_vector;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    #[allow(clippy::print_stdout)]
    {
        println!("initializing the logger...");
    }
    let setup = setup_logging();
    if setup.is_err() {
        #[allow(clippy::panic)]
        {
            paniq!("{:?}", setup);
        }
    }

    log::debug!("-----------------");
    log::debug!("sanitizing constants");
    let sanitizer = sanitize_constants();

    if sanitizer.is_err() {
        #[allow(clippy::panic)]
        {
            paniq!("{:?}", sanitizer);
        }
    }

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

    let system_data_cloned = data.clone();

    let app_state = AppState::new();
    let shared_state = web::Data::new(Mutex::new(app_state));

    HttpServer::new(move || {
        let system_data_cloned_for_spawn = system_data_cloned.clone();

        App::new()
            .wrap(actix_middleware::Logger::default())
            .wrap(
                actix_middleware::DefaultHeaders::new()
                    .header("Permissions-Policy", "interest-cohort=()"),
            )
            .wrap(get_identity_service(
                &*system_data_cloned_for_spawn
                    .config
                    .app_settings
                    .settings
                    .server
                    .cookie_secret,
                &*system_data_cloned_for_spawn
                    .config
                    .app_settings
                    .settings
                    .server
                    .domain,
                system_data_cloned_for_spawn
                    .config
                    .app_settings
                    .settings
                    .server
                    .cookie_max_age_secs,
            ))
            .wrap(actix_middleware::Compress::default())
            .wrap(actix_middleware::NormalizePath::new(
                actix_middleware::TrailingSlash::Trim,
            ))
            .service(api::api_scope(
                &system_data_cloned_for_spawn.config.app_settings.settings.server,
            ))
            .app_data(system_data_cloned_for_spawn)
            .app_data(get_json_err())
            .app_data(shared_state.clone())
            .default_service(web::to(not_found))
    })
    .bind(data.config.app_settings.settings.server.get_uri(false))?
    .run()
    .await?;

    Ok(())
}
