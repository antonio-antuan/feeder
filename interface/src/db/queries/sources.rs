use diesel::prelude::*;
use diesel::{delete, insert_into};

use crate::db::Pool;
use crate::server::result::ApiError;
use crate::schema::{sources, sources_user_settings};
use diesel::sql_types::{Bool, Nullable};
use feeder::models::Source;
use tokio_diesel::*;

sql_function!(fn coalesce(x: Nullable<Bool>, y: Bool) -> Bool);

pub async fn unsubscribe(db_pool: &Pool, source_id: i32, user_id: i32) -> Result<(), ApiError> {
    use crate::schema::{records, records_user_settings};
    let records = records::table.filter(records::source_id.eq(source_id));
    delete(
        records_user_settings::table.filter(
            records_user_settings::record_id
                .eq_any(records.select(records::id))
                .and(records_user_settings::user_id.eq(user_id)),
        ),
    )
    .execute_async(db_pool)
    .await?;
    delete(
        sources_user_settings::table.filter(
            sources_user_settings::source_id
                .eq(source_id)
                .and(sources_user_settings::user_id.eq(user_id)),
        ),
    )
    .execute_async(db_pool)
    .await?;
    Ok(())
}

pub async fn subscribe(db_pool: &Pool, source_id: i32, user_id: i32) -> Result<(), ApiError> {
    insert_into(sources_user_settings::table)
        .values((
            (sources_user_settings::source_id.eq(source_id)),
            (sources_user_settings::user_id.eq(user_id)),
        ))
        .execute_async(db_pool)
        .await?;
    Ok(())
}

pub async fn get_list(db_pool: &Pool, user_id: i32) -> Result<Vec<Source>, ApiError> {
    Ok(sources::table
        .inner_join(sources_user_settings::dsl::sources_user_settings)
        .filter(sources_user_settings::user_id.eq(user_id))
        .select(sources::all_columns)
        .load_async::<Source>(&db_pool)
        .await?)
}
