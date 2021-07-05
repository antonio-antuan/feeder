#[macro_use]
extern crate log;

#[macro_use]
extern crate derive_builder;

extern crate futures;
extern crate serde;

pub mod aggregator;
pub mod config;
pub mod models;
pub mod result;
pub mod storage;
mod updates;
pub use updates::Source;
