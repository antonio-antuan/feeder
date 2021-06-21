#[macro_use]
extern crate log;

pub use rust_tdlib::types::*;

pub mod result;
pub mod tg_client;
mod traits;
pub mod types;
// UpdatesHandler trait implementation
pub mod parsers;
