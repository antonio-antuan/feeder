use super::Storage;
use crate::models;
use crate::result::{Error, Result};
use async_trait::async_trait;
use sqlx::postgres::PgPool;
use std::time::Duration;

pub type Pool = PgPool;

#[derive(Clone)]
pub struct PgStorage {
    pool: Pool,
}

impl PgStorage {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    pub fn migrate(&self) -> Result<()> {
        // TODO
        Ok(())
    }

    pub fn pool(&self) -> Pool {
        self.pool.clone()
    }
}

#[async_trait]
impl Storage for PgStorage {
    async fn save_file(&self, file: models::File) -> Result<()> {
        sqlx::query!(
            "UPDATE files SET local_path = $1, file_name = $2 WHERE id = $3",
            file.local_path,
            file.file_name,
            file.id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn get_file_by_remote_id(&self, remote_id: String) -> Result<Option<models::File>> {
        Ok(sqlx::query_as("SELECT * FROM files WHERE remote_id = $1")
            .bind(remote_id)
            .fetch_optional(&self.pool)
            .await?)
    }

    async fn save_files(&self, files: Vec<models::NewFile>) -> Result<()> {
        for file in files {
            if let Err(e) = sqlx::query!(
                "INSERT INTO files \
                (record_id, kind, local_path, remote_path, remote_id, file_name, type, meta) \
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8) ON CONFLICT DO NOTHING",
                file.record_id,
                file.kind,
                file.local_path,
                file.remote_path,
                file.remote_id,
                file.file_name,
                file.type_,
                file.meta
            )
            .execute(&self.pool)
            .await
            {
                error!("{}", e);
            }
        }
        Ok(())
    }

    async fn set_record_external_link(
        &self,
        source_record_id: String,
        source_id: i32,
        external_link: String,
    ) -> Result<u64> {
        Ok(sqlx::query!(
            "UPDATE records SET external_link = $1 WHERE source_record_id = $2 AND source_id = $3",
            external_link,
            source_record_id,
            source_id
        )
        .execute(&self.pool)
        .await?
        .rows_affected())
    }

    async fn save_records(&self, records: Vec<models::NewRecord>) -> Result<Vec<models::Record>> {
        let mut result: Vec<models::Record> = Vec::with_capacity(records.len());
        for record in records {
            let new_rec = sqlx::query_as!(
                models::Record,
                "INSERT INTO records (title, source_record_id, source_id, content, date, image) \
                VALUES ($1, $2, $3, $4, $5, $6)\
                RETURNING *",
                record.title,
                record.source_record_id,
                record.source_id,
                record.content,
                record.date,
                record.image,
            )
            .fetch_one(&self.pool)
            .await?;
            result.push(new_rec);
        }
        Ok(result)
    }

    async fn set_source_scraped_now(&self, source: models::Source) -> Result<()> {
        sqlx::query!(
            "UPDATE sources SET last_scrape_time = NOW() WHERE id = $1",
            source.id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn search_source(&self, query: &str) -> Result<Vec<models::Source>> {
        let query = sqlx::query_as!(
            models::Source,
            "SELECT * FROM sources \
            WHERE origin ILIKE $1 OR external_link ILIKE $1 OR name ILIKE $1",
            format!("%{}%", query)
        );
        Ok(query.fetch_all(&self.pool).await?)
    }

    async fn get_exact_source(
        &self,
        kind: String,
        origin: String,
    ) -> Result<Option<models::Source>, Error> {
        Ok(sqlx::query_as!(
            models::Source,
            "SELECT * FROM sources WHERE kind = $1 AND origin = $2",
            kind,
            origin
        )
        .fetch_optional(&self.pool)
        .await?)
    }

    async fn get_sources_by_kind(&self, kind: String) -> Result<Vec<models::Source>> {
        Ok(sqlx::query_as!(
            models::Source,
            "SELECT * FROM sources WHERE kind = $1",
            kind
        )
        .fetch_all(&self.pool)
        .await?)
    }

    async fn get_sources_by_kind_for_scrape(
        &self,
        kind: String,
        check_secs_interval: u64,
    ) -> Result<Vec<models::Source>> {
        Ok(sqlx::query_as(
            "SELECT * FROM sources WHERE kind = $1 AND (last_scrape_time < NOW() - $1)",
        )
        .bind(kind)
        .bind(Duration::from_secs(check_secs_interval))
        .fetch_all(&self.pool)
        .await?)
    }

    async fn save_sources(&self, sources: Vec<models::NewSource>) -> Result<Vec<models::Source>> {
        let mut res = Vec::with_capacity(sources.len());
        for s in sources {
            let n = sqlx::query_as!(
                models::Source,
                "INSERT INTO sources (name, origin, kind, image, external_link) VALUES \
                ($1, $2, $3, $4, $5) \
                ON CONFLICT (origin, kind) DO UPDATE 
                    SET name = EXCLUDED.name
                RETURNING *",
                s.name,
                s.origin,
                s.kind,
                s.image,
                s.external_link
            )
            .fetch_one(&self.pool)
            .await?;
            res.push(n)
        }
        Ok(res)
    }
}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        Self::DbError(err.to_string())
    }
}
