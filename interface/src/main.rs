#[macro_use]
extern crate diesel;

use crate::cli::run;

mod cli;
mod init;
mod settings;

mod db;
mod result;
mod schema;

#[cfg(feature = "rest")]
mod rest;

#[tokio::main]
async fn main() {
    env_logger::init();
    settings::init();
    run().await;
}
