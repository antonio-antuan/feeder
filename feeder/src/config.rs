#[derive(Clone, Debug, Builder)]
#[builder(default)]
pub struct AggregatorConfig {
    http: HttpConfig,
    telegram: TelegramConfig,
}

impl AggregatorConfig {
    pub fn http(&self) -> &HttpConfig {
        &self.http
    }

    pub fn telegram(&self) -> &TelegramConfig {
        &self.telegram
    }
}

impl Default for AggregatorConfig {
    fn default() -> Self {
        Self {
            http: HttpConfig::default(),
            telegram: TelegramConfig::default(),
        }
    }
}

#[derive(Clone, Debug, Builder)]
pub struct HttpConfig {
    enabled: bool,
    sleep_secs: u64,
    scrape_source_secs_interval: i32,
}

impl HttpConfig {
    pub fn enabled(&self) -> bool {
        self.enabled
    }
    pub fn sleep_secs(&self) -> u64 {
        self.sleep_secs
    }
    pub fn scrape_source_secs_interval(&self) -> i32 {
        self.scrape_source_secs_interval
    }
}

impl Default for HttpConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            sleep_secs: 60,
            scrape_source_secs_interval: 60,
        }
    }
}

#[derive(Clone, Debug, Builder)]
pub struct TelegramConfig {
    enabled: bool,
    database_directory: String,
    log_verbosity_level: i32,
    api_id: i64,
    api_hash: String,
    phone: String,
    max_download_queue_size: usize,
    files_directory: String,
    log_download_state_secs_interval: u64,
}

impl TelegramConfig {
    pub fn enabled(&self) -> bool {
        self.enabled
    }
    pub fn database_directory(&self) -> &str {
        &self.database_directory
    }
    pub fn log_verbosity_level(&self) -> i32 {
        self.log_verbosity_level
    }
    pub fn api_id(&self) -> i64 {
        self.api_id
    }
    pub fn api_hash(&self) -> &str {
        &self.api_hash
    }
    pub fn phone(&self) -> &str {
        &self.phone
    }
    pub fn max_download_queue_size(&self) -> usize {
        self.max_download_queue_size
    }
    pub fn files_directory(&self) -> &str {
        &self.files_directory
    }
    pub fn log_download_state_secs_interval(&self) -> u64 {
        self.log_download_state_secs_interval
    }
}

impl Default for TelegramConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            database_directory: "tdlib".to_string(),
            log_verbosity_level: 0,
            api_id: 0,
            api_hash: "".to_string(),
            phone: "".to_string(),
            max_download_queue_size: 1,
            files_directory: "".to_string(),
            log_download_state_secs_interval: 0,
        }
    }
}
