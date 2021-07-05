use super::pb::{adapt_source, sources};
use crate::db::queries::sources as sources_queries;
use crate::db::Pool;
use crate::init::App;
use feeder::result::Error;

#[derive(Clone)]
pub struct Service {
    db_pool: Pool,
    aggregator: App,
}

impl Service {
    pub fn new(db_pool: Pool, aggregator: App) -> Self {
        Self {
            db_pool,
            aggregator,
        }
    }
}

#[tonic::async_trait]
impl sources::sources_service_server::SourcesService for Service {
    async fn get_sources_list(
        &self,
        request: tonic::Request<sources::GetSourcesListRequest>,
    ) -> Result<tonic::Response<sources::GetSourcesListResponse>, tonic::Status> {
        let user = super::auth_user(&self.db_pool, request.metadata()).await?;
        let sources = sources_queries::get_for_user(&self.db_pool, user.id).await?;
        Ok(tonic::Response::new(sources::GetSourcesListResponse {
            sources: sources.into_iter().map(From::from).collect(),
        }))
    }

    async fn search_sources(
        &self,
        request: tonic::Request<sources::SearchSourcesRequest>,
    ) -> Result<tonic::Response<sources::SearchSourcesResponse>, tonic::Status> {
        super::auth_user(&self.db_pool, request.metadata()).await?;
        let sources = self
            .aggregator
            .search_source(request.into_inner().query.as_str())
            .await
            .map_err(|e| match e {
                Error::DbError(e) => tonic::Status::internal(e),
                Error::HttpCollectorError(e) => tonic::Status::internal(e.to_string()),
                Error::TgCollectorError(e) => tonic::Status::internal(e.to_string()),
                Error::VkCollectorError(e) => tonic::Status::internal(e.to_string()),
                Error::UpdateNotSupported(e) => tonic::Status::internal(e),
                Error::SourceKindConflict(e) => tonic::Status::internal(e),
                Error::SourceNotFound => tonic::Status::not_found("source not found"),
                Error::SourceCreationError => tonic::Status::internal("cannot create source"),
                Error::IOError(e) => tonic::Status::internal(e.to_string()),
            })?;
        Ok(tonic::Response::new(sources::SearchSourcesResponse {
            sources: sources.into_iter().map(adapt_source).collect(),
        }))
    }

    async fn subscribe(
        &self,
        request: tonic::Request<sources::SubscribeRequest>,
    ) -> Result<tonic::Response<sources::SubscribeResponse>, tonic::Status> {
        let user = super::auth_user(&self.db_pool, request.metadata()).await?;
        sources_queries::subscribe(&self.db_pool, request.into_inner().source_id, user.id).await?;
        Ok(tonic::Response::new(sources::SubscribeResponse {}))
    }

    async fn unsubscribe(
        &self,
        request: tonic::Request<sources::UnsubscribeRequest>,
    ) -> Result<tonic::Response<sources::UnsubscribeResponse>, tonic::Status> {
        let user = super::auth_user(&self.db_pool, request.metadata()).await?;
        sources_queries::unsubscribe(&self.db_pool, request.into_inner().source_id, user.id)
            .await?;
        Ok(tonic::Response::new(sources::UnsubscribeResponse {}))
    }
}
