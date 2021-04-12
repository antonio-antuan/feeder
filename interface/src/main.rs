#[macro_use]
extern crate diesel;

use crate::cli::run;

mod cli;
mod init;
use actix_rt::System;
mod settings;

mod db;
mod result;
mod schema;

#[cfg(feature = "rest")]
mod rest;

#[macro_use]
extern crate diesel_migrations;


fn main() {
    System::with_tokio_rt(|| {
        // build system with a multi-thread tokio runtime.
        tokio::runtime::Builder::new_multi_thread()
            .worker_threads(8)
            .enable_all()
            .build()
            .unwrap()
    })
    .block_on(_main());
}

async fn _main() {
    env_logger::init();
    settings::init();
    run().await;
}
