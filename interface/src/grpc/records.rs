use super::proto::records;
use crate::db::queries::records as records_queries;
use crate::db::Pool;

#[derive(Clone)]
pub struct Service {
    db_pool: Pool,
}

impl Service {
    pub fn new(db_pool: Pool) -> Self {
        Self { db_pool }
    }
}

#[tonic::async_trait]
impl records::records_service_server::RecordsService for Service {
    async fn get_records_list(
        &self,
        request: tonic::Request<records::GetRecordsListRequest>,
    ) -> Result<tonic::Response<records::GetRecordsListResponse>, tonic::Status> {
        let user = super::auth_user(&self.db_pool, request.metadata()).await?;
        let message: records::GetRecordsListRequest = request.into_inner();
        let records = records_queries::get_all_records(
            &self.db_pool,
            user.id,
            match message.source_id {
                0 => None,
                _ => Some(message.source_id),
            },
            match message.record_id {
                0 => None,
                _ => Some(message.record_id),
            },
            message.limit.into(),
            message.offset.into(),
        )
        .await?;
        Ok(tonic::Response::new(records::GetRecordsListResponse {
            records: records.into_iter().map(From::from).collect(),
        }))
    }

    async fn get_records_preview(
        &self,
        request: tonic::Request<records::GetRecordsPreviewRequest>,
    ) -> Result<tonic::Response<records::GetRecordsPreviewResponse>, tonic::Status> {
        let message = request.into_inner();
        let records =
            records_queries::get_filtered(&self.db_pool, message.source_id, 20, 0).await?;
        Ok(tonic::Response::new(records::GetRecordsPreviewResponse {
            records: records
                .into_iter()
                .map(|rec| records::Record {
                    source_record_id: rec.source_record_id,
                    content: rec.content,
                    date: rec.date.timestamp(),
                    id: rec.id,
                    title: rec.title.unwrap_or_default(),
                    source_id: rec.source_id,
                    image: rec.image.unwrap_or_default(),
                })
                .collect(),
        }))
    }

    async fn mark_record(
        &self,
        request: tonic::Request<records::MarkRecordRequest>,
    ) -> Result<tonic::Response<records::MarkRecordResponse>, tonic::Status> {
        let user = super::auth_user(&self.db_pool, request.metadata()).await?;
        let message: records::MarkRecordRequest = request.into_inner();
        let record = records_queries::mark_record(
            &self.db_pool,
            user.id,
            message.record_id,
            message.starred,
        )
        .await?;
        Ok(tonic::Response::new(records::MarkRecordResponse {
            record: Some(record.into()),
        }))
    }
}
