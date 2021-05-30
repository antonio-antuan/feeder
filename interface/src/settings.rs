use config::{Config, ConfigError, Environment, File};
use lazy_static::lazy_static;
use serde::Deserialize;

lazy_static! {
    pub static ref SETTINGS: Settings = Settings::new().expect("can't read settings");
}

pub fn init() {
    lazy_static::initialize(&SETTINGS);
}

#[derive(Debug, Deserialize)]
pub struct Database {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct Server {
    pub host: String,
    pub port: u16,
    pub enabled: bool,
}

#[derive(Debug, Deserialize)]
pub struct HttpCollector {
    pub enabled: bool,
    pub sleep_secs: u64,
    pub scrape_source_secs_interval: i32,
}

#[derive(Debug, Deserialize)]
pub struct TgCollector {
    pub enabled: bool,
    pub database_directory: String,
    pub log_verbosity_level: i32,
    pub encryption_key: String,
    pub api_id: i32,
    pub api_hash: String,
    pub phone: String,
    pub max_download_queue_size: usize,
    pub files_directory: String,
    pub log_download_state_secs_interval: u64,
}

#[derive(Debug, Deserialize)]
pub struct Collectors {
    pub http: HttpCollector,
    pub tg: TgCollector,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub database: Database,
    pub collectors: Collectors,
    pub server: Server,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();

        // Start off by merging in the "default" configuration file
        s.merge(File::with_name("config/default"))?;

        // Add in a local configuration file
        // This file shouldn't be checked in to git
        s.merge(File::with_name("config/local").required(false))?;

        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        s.merge(Environment::with_prefix("app"))?;
        // You can deserialize (and thus freeze) the entire configuration as
        s.try_into()
    }
}
