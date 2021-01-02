use rust_tdlib::types::TdType;
use tokio::sync::mpsc;

#[derive(Debug)]
pub struct Config<'a> {
    pub max_download_queue_size: usize,
    pub log_download_state_secs_interval: u64,
    pub log_verbosity_level: i32,
    pub encryption_key: &'a str,
    pub database_directory: &'a str,
    pub api_id: i64,
    pub api_hash: &'a str,
    pub phone_number: &'a str,
    pub updates_sender: &'a mpsc::Sender<TdType>,
}
