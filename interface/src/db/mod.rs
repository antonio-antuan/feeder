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

pub fn migrate(pool: Pool) -> crate::result::Result<()> {
    // TODO
    Ok(())
}
