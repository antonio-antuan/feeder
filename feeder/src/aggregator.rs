// TODO: no needs for aggregator, handler can be used directly
use crate::models;
use crate::result::Result;
use crate::storage::Storage;
use crate::updates::Source;
use crate::{config, updates};
use std::sync::Arc;
use tg_collector::parsers::TelegramDataParser;

pub struct Aggregator<S, P>
where
    S: Storage + Send + Sync + Clone + 'static,
    P: TelegramDataParser + Send + Sync + Clone + 'static,
{
    handler: updates::SourcesAggregator<S, P>,
}

impl<S, P> Aggregator<S, P>
where
    S: Storage + Send + Sync + Clone + 'static,
    P: TelegramDataParser + Send + Sync + Clone + 'static,
{
    pub fn new(handler: updates::SourcesAggregator<S, P>) -> Self {
        Self { handler }
    }

    pub async fn run(&self) {
        self.handler.run().await
    }

    pub async fn search_source(&self, query: &str) -> Result<Vec<models::Source>> {
        self.handler.search_source(query).await
    }

    pub async fn synchronize(&self, secs_depth: i32, source: Option<Source>) -> Result<()> {
        self.handler.synchronize(secs_depth, source).await
    }
}

pub struct AggregatorBuilder<'a, S, P>
where
    S: Storage + Send + Sync + Clone + 'static,
    P: TelegramDataParser + Send + Sync + Clone + 'static,
{
    config: &'a config::AggregatorConfig,
    storage: S,
    telegram_parser: P,
}

impl<'a, S, P> AggregatorBuilder<'a, S, P>
where
    S: Storage + Clone + Send + Sync + Clone + 'static,
    P: TelegramDataParser + Send + Sync + Clone + 'static,
{
    pub fn new(config: &'a config::AggregatorConfig, storage: S, telegram_parser: P) -> Self {
        Self {
            config,
            storage,
            telegram_parser,
        }
    }

    pub fn build(&self) -> Aggregator<S, P> {
        debug!("config for building: {:?}", self.config);
        let mut updates_builder =
            updates::SourcesAggregator::builder().with_storage(self.storage.clone());

        if self.config.http().enabled() {
            let http_source = updates::http::HttpSource::builder()
                .with_sleep_secs(self.config.http().sleep_secs())
                .with_storage(self.storage.clone())
                .build();
            let http_source = Arc::new(http_source);
            updates_builder = updates_builder.with_http_source(http_source);
        }

        if self.config.telegram().enabled() {
            let tg_source = updates::tg::TelegramSource::builder(
                self.config.telegram().api_id(),
                self.config.telegram().api_hash(),
                self.config.telegram().phone(),
                self.config.telegram().max_download_queue_size(),
                self.config.telegram().files_directory(),
                self.config.telegram().log_download_state_secs_interval(),
                self.telegram_parser.clone(),
            )
            .with_database_directory(self.config.telegram().database_directory())
            .with_log_verbosity_level(self.config.telegram().log_verbosity_level())
            .with_storage(self.storage.clone())
            .build();
            let tg_source = Arc::new(tg_source);
            updates_builder = updates_builder.with_tg_source(tg_source);
        }
        Aggregator::new(updates_builder.build())
    }
}
