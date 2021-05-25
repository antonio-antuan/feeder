use super::handlers;
use crate::db::Pool;
use crate::settings::SETTINGS;
use feeder::aggregator::AggApp;
use feeder::storage::pg::PgStorage;
use std::sync::Arc;
use tg_collector::parsers::DefaultTelegramParser;

pub async fn run_server(
    aggregator: Arc<AggApp<PgStorage, DefaultTelegramParser>>,
    db_pool: Pool,
) -> std::io::Result<()> {
    let api = handlers::records(db_pool);

    Ok(warp::serve(api).run(([127, 0, 0, 1], 3030)).await)
}
