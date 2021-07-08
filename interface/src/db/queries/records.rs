use crate::db::models::RecordWithMeta;
use crate::db::Pool;
use crate::result::Result;
use feeder::models::Record;
use sql_builder::SqlBuilder;

pub async fn get_records(
    db_pool: &Pool,
    user_id: i32,
    source_id: Option<i32>,
    record_id: Option<i32>,
    limit: i64,
    offset: i64,
) -> Result<Vec<RecordWithMeta>> {
    let mut query = SqlBuilder::select_from("records as r");
    query
        .fields(&[
            "r.id",
            "r.title",
            "r.source_record_id as guid",
            "r.source_id",
            "r.content",
            "r.date",
            "r.image",
            "coalesce(rus.starred, false) as starred",
            "array_agg(rt.tag) filter(where rt.tag is not null) as tags",
        ])
        .left()
        .join("records_user_settings as rus")
        .on("r.id = rus.record_id")
        .left()
        .join("record_tags as rt")
        .on("rt.user_id = rus.user_id AND rt.record_id = r.id")
        .left()
        .join("sources_user_settings as sus")
        .on("sus.source_id = r.source_id")
        .left()
        .join("sources as s")
        .on("s.id = sus.source_id")
        .and_where_eq("sus.user_id", user_id)
        .group_by("r.id, rus.starred")
        .limit(limit)
        .offset(offset)
        .order_desc("r.date");
    if let Some(sid) = source_id {
        query.and_where_eq("r.source_id", sid);
    }

    if let Some(rid) = record_id {
        query.and_where_eq("r.id", rid);
    }

    let query = query
        .sql()
        .map_err(|e| crate::result::Error::InternalServerError(e.to_string()))?;
    Ok(sqlx::query_as(query.as_str()).fetch_all(db_pool).await?)
}

pub async fn mark_record(
    db_pool: &Pool,
    user_id: i32,
    record_id: i32,
    starred: bool,
) -> Result<RecordWithMeta> {
    sqlx::query!(
        r#"
    INSERT INTO records_user_settings (record_id, user_id, starred) 
    VALUES ($1, $2, $3)
    ON CONFLICT (user_id, record_id) 
    DO UPDATE SET 
        starred = EXCLUDED.starred"#,
        record_id,
        user_id,
        starred
    )
    .execute(db_pool)
    .await?;

    Ok(get_records(db_pool, user_id, None, Some(record_id), 1, 0)
        .await?
        .first()
        .cloned()
        .unwrap())
}

pub async fn get_tags(db_pool: &Pool, user_id: i32, record_id: i32) -> Result<Vec<String>> {
    let record = sqlx::query!(
        "SELECT array_agg(tag) as tags FROM record_tags WHERE user_id = $1 AND record_id = $2",
        user_id,
        record_id
    )
    .fetch_one(db_pool)
    .await?;
    Ok(record.tags.unwrap_or_default())
}

pub async fn add_tag(db_pool: &Pool, user_id: i32, record_id: i32, tag: String) -> Result<()> {
    sqlx::query!(
        r#"
    INSERT INTO record_tags (record_id, user_id, tag)
    VALUES ($1, $2, $3)
    ON CONFLICT (record_id, user_id) DO NOTHING
        "#,
        record_id,
        user_id,
        tag
    )
    .execute(db_pool)
    .await?;
    Ok(())
}

pub async fn remove_tag(db_pool: &Pool, user_id: i32, record_id: i32, tag: String) -> Result<()> {
    sqlx::query!(
        r#"
    DELETE FROM record_tags
    WHERE record_id = $1 AND user_id = $2 and tag = $3"#,
        record_id,
        user_id,
        tag
    )
    .execute(db_pool)
    .await?;
    Ok(())
}

pub async fn get_by_source_id(
    db_pool: &Pool,
    source_id: i32,
    limit: i64,
    offset: i64,
) -> Result<Vec<Record>> {
    Ok(sqlx::query_as!(
        Record,
        "SELECT * FROM records WHERE source_id = $1 ORDER BY date DESC limit $2 offset $3",
        source_id,
        limit,
        offset
    )
    .fetch_all(db_pool)
    .await?)
}
