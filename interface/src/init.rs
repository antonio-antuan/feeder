use crate::settings;

use crate::db;
use feeder::aggregator;
use feeder::aggregator::AggApp;
use feeder::config;
use feeder::storage::pg::PgStorage;
use std::sync::Arc;
use tg_collector::parsers::DefaultTelegramParser;

fn init_app_config() -> config::AppConfig {
    let http_config = config::HttpConfigBuilder::default()
        .enabled(settings::SETTINGS.collectors.http.enabled)
        .sleep_secs(settings::SETTINGS.collectors.http.sleep_secs)
        .scrape_source_secs_interval(
            settings::SETTINGS
                .collectors
                .http
                .scrape_source_secs_interval,
        )
        .build()
        .expect("can't create http collector config");
    let tg_config = config::TelegramConfigBuilder::default()
        .enabled(settings::SETTINGS.collectors.tg.enabled)
        .database_directory(settings::SETTINGS.collectors.tg.database_directory.clone())
        .files_directory(settings::SETTINGS.collectors.tg.files_directory.clone())
        .log_verbosity_level(settings::SETTINGS.collectors.tg.log_verbosity_level)
        .max_download_queue_size(settings::SETTINGS.collectors.tg.max_download_queue_size)
        .encryption_key(settings::SETTINGS.collectors.tg.encryption_key.clone())
        .phone(settings::SETTINGS.collectors.tg.phone.clone())
        .api_hash(settings::SETTINGS.collectors.tg.api_hash.clone())
        .log_download_state_secs_interval(
            settings::SETTINGS
                .collectors
                .tg
                .log_download_state_secs_interval,
        )
        .api_id(settings::SETTINGS.collectors.tg.api_id)
        .build()
        .expect("can't create telegram collector config");
    let vk_config = config::VkConfigBuilder::default()
        .enabled(settings::SETTINGS.collectors.vk.enabled)
        .token(settings::SETTINGS.collectors.vk.token.clone())
        .sleep_secs(settings::SETTINGS.collectors.vk.sleep_secs)
        .scrape_source_secs_interval(settings::SETTINGS.collectors.vk.scrape_source_secs_interval)
        .build()
        .expect("can't create vk collector config");
    config::AppConfigBuilder::default()
        .http(http_config)
        .telegram(tg_config)
        .vk(vk_config)
        .build()
        .expect("can't create collector config")
}

pub type App = Arc<AggApp<PgStorage, DefaultTelegramParser>>;

pub fn build_app() -> App {
    let db_pool = db::init_pool(settings::SETTINGS.database.url.as_str());
    let storage = PgStorage::new(db_pool.clone());
    let app_config = init_app_config();
    Arc::new(
        aggregator::AppBuilder::new(&app_config, storage.clone(), DefaultTelegramParser::new())
            .build(),
    )
}
