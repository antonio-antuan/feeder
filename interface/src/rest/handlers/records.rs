use crate::db::models::User;
use crate::db::queries::records as records_queries;
use crate::db::Pool;
use crate::rest::filters::{with_db, with_user};
use crate::result::Result;
use std::convert::Infallible;
use warp::Filter;

#[derive(Debug, Deserialize)]
pub struct SourceFilter {
    pub source_id: i32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RecordsQuery {
    All,
    Starred,
}

#[derive(Debug, Deserialize)]
pub struct GetFilteredRecordsRequest {
    pub source_id: Option<i32>,
    pub query: RecordsQuery,
    pub limit: i64,
    pub offset: i64,
}

#[derive(Debug, Deserialize)]
pub struct MarkRecord {
    starred: bool,
}

pub fn records(
    db: Pool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    records_preview(db.clone())
        .or(records_list(db.clone()))
        .or(mark_record(db.clone()))
}

fn records_list(
    db: Pool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("records/preview")
        .and(warp::get())
        .and(warp::query::<models::GetFilteredRecordsRequest>())
        .and(with_db(db))
        .and_then(get_records_handler)
}

fn mark_record(
    db: Pool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("records" / i32)
        .and(warp::post())
        .and(warp::body::json::<MarkRecord>())
        .and(with_db(db))
        .and_then(mark_record_handler)
}

fn records_preview(
    db: Pool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("records")
        .and(warp::get())
        .and(warp::query::<models::SourceFilter>())
        .and(with_db(db))
        .and_then(records_preview_handler)
}

async fn get_records_handler(
    db_pool: Pool,
    params: GetFilteredRecordsRequest,
) -> Result<impl warp::Reply, warp::Rejection> {
    let records = match params.query {
        RecordsQuery::All => {
            records_queries::get_all_records(
                &db_pool,
                user.id,
                params.source_id,
                None,
                params.limit,
                params.offset,
            )
            .await
        }
        RecordsQuery::Starred => {
            records_queries::get_starred_records(
                &db_pool,
                user.id,
                params.source_id,
                params.limit,
                params.offset,
            )
            .await
        }
    };
    Ok(warp::reply::json(&records))
}

async fn mark_record_handler(
    record_id: i32,
    params: MarkRecord,
    db_pool: Pool,
) -> Result<impl warp::Reply, warp::Rejection> {
    let r = records_queries::mark_record(&db_pool, user.id, record_id, params.starred).await?;
    Ok(warp::reply::json(&r))
}

async fn records_preview_handler(
    db_pool: Pool,
    params: SourceFilter,
) -> Result<impl warp::Reply, warp::Rejection> {
    let res = records_queries::get_filtered(&db_pool, params.source_id, 20, 0).await?;
    Ok(warp::reply::json(&res))
}
