#[macro_use]
extern crate log;
#[macro_use]
extern crate derive_builder;

#[cfg(feature = "pg-storage")]
#[macro_use]
extern crate diesel;

#[cfg(feature = "pg-storage")]
#[macro_use]
extern crate diesel_migrations;

extern crate futures;
extern crate serde;

pub mod aggregator;
pub mod config;
pub mod models;
pub mod result;
pub mod storage;
mod updates;
