use crate::cli::run;

mod cli;
mod init;

mod settings;

mod auth;
mod db;
mod grpc;
mod result;

#[tokio::main]
async fn main() {
    env_logger::init();
    settings::init();
    run().await;
}
