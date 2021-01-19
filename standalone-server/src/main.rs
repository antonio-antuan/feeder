#[macro_use]
extern crate diesel;
mod cli;
mod init;
mod settings;

use crate::cli::run;

mod db;
mod result;
mod schema;
mod server;

#[actix_rt::main]
async fn main() {
    env_logger::init();
    settings::init();
    run().await;
}
