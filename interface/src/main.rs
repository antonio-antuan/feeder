#[macro_use]
extern crate diesel;

use crate::cli::run;

mod cli;
mod init;

mod settings;

mod auth;
mod db;
mod grpc;
mod proto;
mod result;
mod schema;

#[macro_use]
extern crate diesel_migrations;

#[tokio::main]
async fn main() {
    env_logger::init();
    settings::init();
    run().await;
}
