#[macro_use]
extern crate log;

pub mod config;
pub mod parsers;
pub mod result;
pub mod tg_client;
mod traits;
pub mod types;

pub use rust_tdlib::types::*;
