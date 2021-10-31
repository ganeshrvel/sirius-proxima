#![warn(clippy::all)]
#![warn(
clippy::print_literal,
clippy::print_with_newline,
clippy::println_empty_string
)]

#[macro_use]
mod macros;

use crate::common::errors::setup_errors::SetupError;
use crate::utils::logs::fern_log::setup_logging;
use rust_gpiozero::LED;
use crate::common::models::data::AppData;

mod common;
mod constants;
mod helpers;
mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let s = setup_logging();

    if let Err(e) = s {
        return Err(SetupError::LoggerError(e, "P00001").into());
    }

    if let Err(e) = run().await {
        log::error!("{:?}", e);

        return Err(e);
    }

    Ok(())
}

async fn run() -> anyhow::Result<()> {
    // let data = AppData::new().await?;
    // let data = actix_web::web::Data::new(data);

    //  println!("{:?}", data.config);

    Ok(())
}
