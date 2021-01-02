use crate::models;
use crate::result::Result;
use async_trait::async_trait;

#[cfg(feature = "pg-storage")]
pub mod pg;

#[cfg(feature = "pg-storage")]
pub mod schema;

#[async_trait]
pub trait Storage {
    async fn save_file(&self, file: models::File) -> Result<()>;
    async fn get_file_by_remote_id(&self, remote_id: String) -> Result<Option<models::File>>;
    async fn save_files(&self, files: Vec<models::NewFile>) -> Result<()>;

    async fn set_record_external_link(
        &self,
        source_record_id: String,
        source_id: i32,
        external_link: String,
    ) -> Result<usize>;
    async fn save_records(&self, records: Vec<models::NewRecord>) -> Result<Vec<models::Record>>;

    async fn set_source_scraped_now(&self, source: models::Source) -> Result<()>;
    async fn search_source(&self, query: &str) -> Result<Vec<models::Source>>;
    async fn get_sources_by_kind(&self, kind: String) -> Result<Vec<models::Source>>;
    async fn get_sources_by_kind_for_scrape(
        &self,
        kind: String,
        check_secs_interval: &i32,
    ) -> Result<Vec<models::Source>>;
    async fn save_sources(&self, sources: Vec<models::NewSource>) -> Result<Vec<models::Source>>;
}
