use crate::db::Pool;
use crate::rest::auth::Authorization;
use crate::rest::handlers::routes::routes;
use crate::settings::SETTINGS;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use feeder::aggregator::AggApp;
use feeder::storage::pg::PgStorage;
use std::sync::Arc;
use tg_collector::parsers::DefaultTelegramParser;

pub async fn run_server(
    aggregator: Arc<AggApp<PgStorage, DefaultTelegramParser>>,
    db_pool: Pool,
) -> std::io::Result<()> {
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Authorization::default())
            .wrap(Logger::default())
            .configure(routes)
            .app_data(Data::new(aggregator.clone()))
            .app_data(Data::new(db_pool.clone()))
    });

    server
        .bind(format!("{}:{}", SETTINGS.server.host, SETTINGS.server.port))?
        .run()
        .await
}
