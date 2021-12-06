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
use crate::constants::header_keys::HeaderKeys;
use crate::constants::strings::Strings;
use crate::helpers::actix::actix_helpers::{
    get_identity_service, get_json_err, make_server_config,
};
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

    // app data
    let app_data = AppData::new().await?;
    let shared_app_data = actix_web::web::Data::new(app_data);

    let server = shared_app_data.config.app_settings.settings.server.clone();
    let server_url_with_protocol = server.get_uri(true)?;
    let server_url_without_protocol = server.get_uri(false)?;
    let cookie_secret = server.cookie_secret.clone();
    let domain = server.get_domain()?;
    let cookie_max_age_secs = server.cookie_max_age_secs;
    let enable_https = server.https;
    let tls = server.tls.clone();
    log::info!("starting the server on: {}", server_url_with_protocol);

    // app state
    let app_state = AppState::new();
    let shared_state = web::Data::new(Mutex::new(app_state));

    let http_server_base = HttpServer::new(move || {
        App::new()
            .wrap(actix_middleware::Logger::default())
            .wrap(
                actix_middleware::DefaultHeaders::new()
                    .header(HeaderKeys::PERMISSIONS_POLICY, "interest-cohort=()"),
            )
            .wrap(actix_middleware::Compress::default())
            .wrap(actix_middleware::NormalizePath::new(
                actix_middleware::TrailingSlash::Trim,
            ))
            .wrap(get_identity_service(
                cookie_secret.as_str(),
                domain.as_str(),
                cookie_max_age_secs,
            ))
            .service(api::api_scope(&server))
            .app_data(shared_app_data.clone())
            .app_data(get_json_err())
            .app_data(shared_state.clone())
            .default_service(web::to(not_found))
    });

    let http_server_binding;

    if enable_https {
        if let Some(t) = tls {
            let tls_cfg = make_server_config(&t)?;

            http_server_binding =
                http_server_base.bind_rustls(server_url_without_protocol, tls_cfg)?;
        } else {
            http_server_binding = http_server_base.bind(server_url_without_protocol)?;
        }
    } else {
        http_server_binding = http_server_base.bind(server_url_without_protocol)?;
    }

    http_server_binding.run().await?;

    Ok(())
}
