use diesel::prelude::*;
use diesel::{delete, insert_into, update};

use crate::db::{models::SourceWithMeta, Pool};
use crate::result::{Error, Result};
use crate::schema::{sources, sources_user_settings, user_folders, user_source_to_folder};
use diesel::dsl::min;
use diesel::sql_types::{Bool, Nullable};
use tokio_diesel::*;

sql_function!(fn coalesce(x: Nullable<Bool>, y: Bool) -> Bool);

pub async fn unsubscribe(db_pool: &Pool, source_id: i32, user_id: i32) -> Result<()> {
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

pub async fn subscribe(db_pool: &Pool, source_id: i32, user_id: i32) -> Result<()> {
    let folder_id: i32 = match user_folders::table
        .filter(user_folders::user_id.eq(user_id))
        .select(min(user_folders::id))
        .get_result_async(&db_pool)
        .await?
    {
        None => {
            return Err(Error::InternalServerError(
                "user folders not found".to_string(),
            ))
        }
        Some(v) => v,
    };
    let user_source_id: i32 = insert_into(sources_user_settings::table)
        .values((
            (sources_user_settings::source_id.eq(source_id)),
            (sources_user_settings::user_id.eq(user_id)),
        ))
        .returning(sources_user_settings::id)
        .get_result_async(db_pool)
        .await?;
    insert_into(user_source_to_folder::table)
        .values((
            (user_source_to_folder::user_source_id.eq(user_source_id)),
            (user_source_to_folder::folder_id.eq(folder_id)),
        ))
        .execute_async(db_pool)
        .await?;
    Ok(())
}

pub async fn get_list(db_pool: &Pool, user_id: i32) -> Result<Vec<SourceWithMeta>> {
    Ok(sources::table
        .inner_join(
            sources_user_settings::dsl::sources_user_settings
                .inner_join(user_source_to_folder::dsl::user_source_to_folder),
        )
        .filter(sources_user_settings::user_id.eq(user_id))
        .select((
            sources::id,
            sources::name,
            sources::origin,
            sources::kind,
            sources::image,
            sources::last_scrape_time,
            sources::external_link,
            user_source_to_folder::folder_id,
        ))
        .load_async::<SourceWithMeta>(&db_pool)
        .await?)
}

pub async fn move_to_folder(
    db_pool: &Pool,
    user_id: i32,
    source_id: i32,
    folder_id: i32,
) -> Result<()> {
    let sut: i32 = sources_user_settings::table
        .filter(
            sources_user_settings::source_id
                .eq(source_id)
                .and(sources_user_settings::user_id.eq(user_id)),
        )
        .select(sources_user_settings::id)
        .get_result_async(&db_pool)
        .await?;
    update(user_source_to_folder::table.filter(user_source_to_folder::user_source_id.eq(sut)))
        .set(user_source_to_folder::folder_id.eq(folder_id))
        .execute_async(&db_pool)
        .await?;
    Ok(())
}
