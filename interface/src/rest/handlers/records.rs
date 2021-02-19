use crate::db::models::{RecordWithMeta, User};
use crate::db::queries::records as records_queries;
use crate::db::Pool;
use crate::result::Result;
use actix_web::web::{Data, Json, Path, Query};
use feeder::models::Record;
use serde::Deserialize;

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
pub struct SourceFilter {
    pub source_id: i32,
}

pub async fn get_records_for_preview(
    db_pool: Data<Pool>,
    params: Query<SourceFilter>,
) -> Result<Json<Vec<Record>>> {
    Ok(Json(
        records_queries::get_filtered(&db_pool, params.source_id, 20, 0).await?,
    ))
}

pub async fn get_records(
    db_pool: Data<Pool>,
    params: Query<GetFilteredRecordsRequest>,
    user: User,
) -> Result<Json<Vec<RecordWithMeta>>> {
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
    Ok(Json(records?))
}

#[derive(Debug, Deserialize)]
pub struct MarkRecord {
    starred: bool,
}

pub async fn mark_record(
    db_pool: Data<Pool>,
    record_id: Path<i32>,
    params: Json<MarkRecord>,
    user: User,
) -> Result<Json<RecordWithMeta>> {
    Ok(Json(
        records_queries::mark_record(&db_pool, user.id, record_id.into_inner(), params.starred).await?,
    ))
}

// TODO: retrieve comments for all records (if exists)
// make requests inplace or store in db???
