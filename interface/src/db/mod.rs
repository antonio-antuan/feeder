pub mod models;
pub mod queries;

use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool as _Pool},
};
pub type Pool = _Pool<ConnectionManager<PgConnection>>;

pub fn init_pool(db_url: &str) -> Pool {
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    Pool::builder()
        .build(manager)
        .expect("can't initialize pool")
}

embed_migrations!();

pub fn migrate(pool: Pool) -> Result<(), diesel_migrations::RunMigrationsError> {
    let connection = pool.get().expect("can't get connection from pool");
    embedded_migrations::run(&connection)?;
    Ok(())
}
