#![warn(clippy::all)]
#![warn(
    clippy::print_literal,
    clippy::print_with_newline,
    clippy::println_empty_string
)]

#[macro_use]
mod macros;

#[macro_use]
extern crate rocket;

use crate::common::errors::setup_errors::SetupError;
use crate::common::models::data::AppData;
use crate::utils::logs::fern_log::setup_logging;
use rocket::fairing::AdHoc;
use rocket::{Build, Rocket};
// use rust_gpiozero::LED;

use rocket::figment::providers::{Env, Format, Serialized, Toml};
use rocket::figment::{Figment, Profile};
use rocket::serde::Deserialize;
use rocket::{Config, State};

mod common;
mod constants;
mod helpers;
mod utils;

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
struct AppConfig {
    key: String,
    port: u16,
}

#[rocket::main]
async fn main() -> anyhow::Result<()> {
    let s = setup_logging();

    if let Err(e) = s {
        paniq!("[P00001] failed to initialize the logger: {:?}", e);
    }

    if let Err(e) = run().await {
        log::error!("{:?}", e);

        return Err(e);
    }

    Ok(())
}

async fn run() -> anyhow::Result<()> {
    let figment = Figment::from(rocket::Config::default());

    let r = rocket::custom(figment)
        .mount("/", routes![index])
        .launch()
        .await?;

    Ok(r)
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}
