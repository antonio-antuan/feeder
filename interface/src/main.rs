#[macro_use]
extern crate diesel;

use crate::cli::run;

mod cli;
mod init;
use crate::rest::wrapped_spawn;
use actix_rt::System;

mod settings;

mod db;
mod result;
mod schema;

#[cfg(feature = "rest")]
mod rest;

#[macro_use]
extern crate diesel_migrations;

#[tokio::main]
async fn main() {
    env_logger::init();
    settings::init();
    run();
}
