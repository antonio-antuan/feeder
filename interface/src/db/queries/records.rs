use diesel::insert_into;
use diesel::prelude::*;
use diesel::Queryable;

use crate::db::models::RecordWithMeta;
use crate::db::Pool;
use crate::server::result::ApiError;
use crate::schema::{records, records_user_settings, sources, sources_user_settings};
use diesel::pg::upsert::excluded;
use diesel::sql_types::{Bool, Nullable};
use feeder::models::Record;
use tokio_diesel::*;

sql_function!(fn coalesce(x: Nullable<Bool>, y: Bool) -> Bool);

#[derive(Queryable)]
struct Meta {
    pub starred: bool,
}

pub async fn get_starred_records(
    db_pool: &Pool,
    user_id: i32,
    source_id: Option<i32>,
    limit: i64,
    offset: i64,
) -> Result<Vec<RecordWithMeta>, ApiError> {
    let query = records::table
        .inner_join(records_user_settings::dsl::records_user_settings)
        .filter(
            records_user_settings::user_id
                .eq(user_id)
                .and(records_user_settings::starred),
        )
        .order(records::date.desc())
        .limit(limit)
        .offset(offset)
        .select((
            records::id,
            records::title,
            records::source_record_id,
            records::source_id,
            records::content,
            records::date,
            records::image,
            records_user_settings::starred.nullable(),
        ));

    let records = match source_id {
        Some(source_id) => {
            query
                .filter(records::source_id.eq(source_id))
                .load_async::<RecordWithMeta>(db_pool)
                .await
        }
        None => query.load_async::<RecordWithMeta>(db_pool).await,
    };
    Ok(records?)
}

pub async fn get_all_records(
    db_pool: &Pool,
    user_id: i32,
    source_id: Option<i32>,
    record_id: Option<i32>,
    limit: i64,
    offset: i64,
) -> Result<Vec<RecordWithMeta>, ApiError> {
    let query = records::table
        .left_join(records_user_settings::dsl::records_user_settings)
        .inner_join(
            sources::dsl::sources.inner_join(sources_user_settings::dsl::sources_user_settings),
        )
        .filter(sources_user_settings::user_id.eq(user_id))
        .order(records::date.desc())
        .limit(limit)
        .offset(offset)
        .select((
            records::id,
            records::title,
            records::source_record_id,
            records::source_id,
            records::content,
            records::date,
            records::image,
            records_user_settings::starred.nullable(),
        ));
    let records = match (source_id, record_id) {
        (None, None) => query.load_async::<RecordWithMeta>(db_pool).await,
        (Some(s), Some(r)) => {
            query
                .filter(records::source_id.eq(s).and(records::id.eq(r)))
                .load_async::<RecordWithMeta>(db_pool)
                .await
        }
        (Some(s), None) => {
            query
                .filter(records::source_id.eq(s))
                .load_async::<RecordWithMeta>(db_pool)
                .await
        }
        (None, Some(r)) => {
            query
                .filter(records::id.eq(r))
                .load_async::<RecordWithMeta>(db_pool)
                .await
        }
    };
    Ok(records?)
}

pub async fn mark_record(
    db_pool: &Pool,
    user_id: i32,
    record_id: i32,
    starred: bool,
) -> Result<RecordWithMeta, ApiError> {
    let starred = records_user_settings::starred.eq(coalesce(starred, false));

    insert_into(records_user_settings::table)
        .values((
            records_user_settings::record_id.eq(record_id),
            records_user_settings::user_id.eq(user_id),
            starred,
        ))
        .on_conflict((
            records_user_settings::user_id,
            records_user_settings::record_id,
        ))
        .do_update()
        .set((records_user_settings::starred.eq(excluded(records_user_settings::starred)),))
        .execute_async(db_pool)
        .await?;
    Ok(
        get_all_records(db_pool, user_id, None, Some(record_id), 1, 0)
            .await?
            .first()
            .cloned()
            .unwrap(),
    )
}

pub async fn get_filtered(
    db_pool: &Pool,
    source_id: i32,
    limit: i64,
    offset: i32,
) -> Result<Vec<Record>, ApiError> {
    Ok(records::table
        .filter(records::source_id.eq(source_id))
        .order(records::date.desc())
        .limit(limit)
        .offset(offset.into())
        .load_async::<Record>(db_pool)
        .await?)
}
