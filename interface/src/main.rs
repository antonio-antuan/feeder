#[macro_use]
extern crate diesel;

use crate::cli::run;
use crate::grpc::server::run_server;

mod cli;
mod init;

mod settings;

mod auth;
mod db;
mod grpc;
mod result;
mod schema;

#[macro_use]
extern crate diesel_migrations;

#[tokio::main]
async fn main() {
    env_logger::init();
    settings::init();
    let app = init::build_app();

    run_server("0.0.0.0:8001", app).await.unwrap();
    run().await;
}
