#[macro_use]
extern crate diesel;
#[macro_use]
extern crate log;

use feeder::aggregator;
use feeder::config;
use feeder::storage::pg::PgStorage;
use tg_collector::parsers::DefaultTelegramParser;
mod settings;
use settings::SETTINGS;
use std::sync::Arc;
use tokio::time::Duration;

mod db;
mod result;
mod schema;
mod server;

#[actix_rt::main]
async fn main() {
    env_logger::init();
    settings::init();
    let db_pool = db::init_pool(settings::SETTINGS.database.url.as_str());
    let storage = PgStorage::new(db_pool.clone());
    storage.migrate().expect("migrations failed");
    let http_config = config::HttpConfigBuilder::default()
        .enabled(SETTINGS.collectors.http.enabled)
        .sleep_secs(SETTINGS.collectors.http.sleep_secs)
        .scrape_source_secs_interval(SETTINGS.collectors.http.scrape_source_secs_interval)
        .build()
        .expect("can't build http config");
    let tg_config = config::TelegramConfigBuilder::default()
        .enabled(SETTINGS.collectors.tg.enabled)
        .database_directory(SETTINGS.collectors.tg.database_directory.clone())
        .files_directory(SETTINGS.collectors.tg.files_directory.clone())
        .log_verbosity_level(SETTINGS.collectors.tg.log_verbosity_level)
        .max_download_queue_size(SETTINGS.collectors.tg.max_download_queue_size)
        .encryption_key(SETTINGS.collectors.tg.encryption_key.clone())
        .phone(SETTINGS.collectors.tg.phone.clone())
        .api_hash(SETTINGS.collectors.tg.api_hash.clone())
        .log_download_state_secs_interval(SETTINGS.collectors.tg.log_download_state_secs_interval)
        .api_id(SETTINGS.collectors.tg.api_id)
        .build()
        .expect("can't build telegram config");
    let agg_config = config::AggregatorConfigBuilder::default()
        .http(http_config)
        .telegram(tg_config)
        .build()
        .expect("can't build aggregator config");
    let aggregator = Arc::new(
        aggregator::AggregatorBuilder::new(
            &agg_config,
            storage.clone(),
            DefaultTelegramParser::new(),
        )
        .build(),
    );
    let agg_runner = aggregator.clone();
    tokio::spawn(async move { agg_runner.run().await });

    if SETTINGS.collectors.sync.before_start {
        aggregator
            .synchronize(SETTINGS.collectors.sync.secs_depth, None)
            .await
            .expect("can't synchronize");
    };
    if SETTINGS.server.enabled {
        server::server::server(aggregator, db_pool)
            .await
            .expect("can't run server");
    }
}
