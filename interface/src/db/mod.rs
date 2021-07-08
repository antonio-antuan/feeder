pub mod models;
pub mod queries;
use sqlx::postgres::{PgPool, PgPoolOptions};

pub type Pool = PgPool;

pub async fn init_pool(db_url: &str) -> Pool {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await
        .expect("can't initialize pool")
}

pub async fn migrate(db_pool: Pool) {
    sqlx::migrate!()
        .set_ignore_missing(true)
        .run(&db_pool)
        .await
        .expect("can't migrate");
}
