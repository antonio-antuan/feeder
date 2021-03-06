use diesel::prelude::*;
use diesel::{delete, insert_into};

use crate::db::models::RecordWithMeta;
use crate::db::Pool;
use crate::result::Result;
use crate::schema::{record_tags, records, records_user_settings, sources, sources_user_settings};
use diesel::pg::upsert::excluded;
use diesel::sql_types::{Array, Bool, Nullable, Text};
use feeder::models::Record;
use tokio_diesel::*;

sql_function!(fn coalesce_bool(x: Nullable<Bool>, y: Bool) -> Bool);
sql_function!(fn coalesce_array(x: Nullable<Array<Text>>, y: Array<Text>) -> Array<Text>);

sql_function!(fn array_agg(x: Nullable<Text>) -> Array<Nullable<Text>>);

pub async fn get_starred_records(
    db_pool: &Pool,
    user_id: i32,
    source_id: Option<i32>,
    limit: i64,
    offset: i64,
) -> Result<Vec<RecordWithMeta>> {
    let query = records::table
        .inner_join(records_user_settings::dsl::records_user_settings)
        .left_join(record_tags::dsl::record_tags)
        .filter(
            records_user_settings::user_id
                .eq(user_id)
                .and(records_user_settings::starred),
        )
        .group_by((records::id, records_user_settings::starred))
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
            array_agg(record_tags::tag.nullable()),
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
) -> Result<Vec<RecordWithMeta>> {
    let query = records::table
        .left_join(records_user_settings::dsl::records_user_settings)
        .left_join(record_tags::dsl::record_tags)
        .inner_join(
            sources::dsl::sources.inner_join(sources_user_settings::dsl::sources_user_settings),
        )
        .filter(sources_user_settings::user_id.eq(user_id))
        .group_by((records::id, records_user_settings::starred))
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
            array_agg(record_tags::tag.nullable()),
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
) -> Result<RecordWithMeta> {
    let starred = records_user_settings::starred.eq(coalesce_bool(starred, false));

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

pub async fn add_tag(db_pool: &Pool, user_id: i32, record_id: i32, tag: String) -> Result<()> {
    insert_into(record_tags::table)
        .values((
            record_tags::record_id.eq(record_id),
            record_tags::user_id.eq(user_id),
            record_tags::tag.eq(tag),
        ))
        .on_conflict_do_nothing()
        .execute_async(db_pool)
        .await?;
    Ok(())
}

pub async fn remove_tag(db_pool: &Pool, user_id: i32, record_id: i32, tag: String) -> Result<()> {
    delete(
        record_tags::table.filter(
            record_tags::record_id
                .eq(record_id)
                .and(record_tags::user_id.eq(user_id))
                .and(record_tags::tag.eq(tag)),
        ),
    )
    .execute_async(db_pool)
    .await?;
    Ok(())
}

pub async fn get_filtered(
    db_pool: &Pool,
    source_id: i32,
    limit: i64,
    offset: i32,
) -> Result<Vec<Record>> {
    Ok(records::table
        .filter(records::source_id.eq(source_id))
        .order(records::date.desc())
        .limit(limit)
        .offset(offset.into())
        .load_async::<Record>(db_pool)
        .await?)
}
