use super::schema::{files, records, sources};
use super::Storage;
use crate::models;
use crate::result::{Error, Result};
use async_trait::async_trait;

use diesel::dsl::IntervalDsl;
use diesel::expression::functions::date_and_time::now;

use diesel::pg::upsert::excluded;
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool as _Pool},
    update,
};
use std::collections::HashMap;
use tokio_diesel::*;

pub type Pool = _Pool<ConnectionManager<PgConnection>>;

embed_migrations!();

#[derive(Clone)]
pub struct PgStorage {
    pool: Pool,
}

impl PgStorage {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    pub fn migrate(&self) -> Result<(), diesel_migrations::RunMigrationsError> {
        let connection = self.pool.get().expect("can't get connection from pool");
        embedded_migrations::run(&connection)?;
        Ok(())
    }
}

#[async_trait]
impl Storage for PgStorage {
    async fn save_file(&self, file: models::File) -> Result<()> {
        diesel::update(files::table.filter(files::id.eq(file.id)))
            .set((
                files::local_path.eq(file.local_path.clone()),
                files::file_name.eq(file.file_name.clone()),
            ))
            .execute_async(&self.pool)
            .await?;
        Ok(())
    }

    async fn get_file_by_remote_id(&self, remote_id: String) -> Result<Option<models::File>> {
        match files::table
            .filter(files::remote_id.eq(remote_id.clone()))
            .get_results_async(&self.pool)
            .await
        {
            Ok(mut found_files) => match found_files.len() {
                0 => Ok(None),
                1 => Ok(found_files.pop()),
                _ => Err(Error::DbError(format!(
                    "found multiple files for id {}",
                    remote_id
                ))),
            },
            Err(tokio_diesel::AsyncError::Error(diesel::NotFound)) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    async fn save_files(&self, files: Vec<models::NewFile>) -> Result<()> {
        let mut key_to_files: HashMap<(String, i32), Vec<models::NewFile>> = HashMap::new();
        for file in files {
            let key = (file.remote_id.clone().unwrap(), file.record_id);
            let existed = key_to_files.get_mut(&key);
            match existed {
                Some(files) => {
                    files.push(file);
                }
                None => {
                    key_to_files.insert(key, vec![file]);
                }
            }
        }
        for file in files::table
            .filter(
                files::remote_id.eq_any(
                    key_to_files
                        .keys()
                        .into_iter()
                        .map(|(ri, _)| ri.clone())
                        .collect::<Vec<String>>(),
                ),
            )
            .load_async::<models::File>(&self.pool)
            .await?
        {
            let file_id = file.id;
            key_to_files
                .remove(&(file.remote_id.unwrap(), file.record_id))
                .map(|files| async move {
                    for file in files {
                        let update_result =
                            diesel::update(files::table.filter(files::id.eq(file_id)))
                                .set((
                                    files::local_path.eq(file.local_path.clone()),
                                    files::file_name.eq(file.file_name.clone()),
                                ))
                                .execute_async(&self.pool)
                                .await;
                        match update_result {
                            Ok(_) => {}
                            Err(e) => error!("{}", e),
                        }
                    }
                });
        }
        let mut to_insert = vec![];
        for files in key_to_files.values() {
            to_insert.extend(files.clone())
        }
        diesel::insert_into(files::table)
            .values(to_insert)
            .on_conflict(files::remote_id)
            .do_nothing()
            .execute_async(&self.pool)
            .await?;
        Ok(())
    }

    async fn set_record_external_link(
        &self,
        source_record_id: String,
        source_id: i32,
        external_link: String,
    ) -> Result<usize> {
        Ok(diesel::update(
            records::table.filter(
                records::source_record_id
                    .eq(source_record_id)
                    .and(records::source_id.eq(source_id)),
            ),
        )
        .set(records::external_link.eq(external_link.clone()))
        .execute_async(&self.pool)
        .await?)
    }

    async fn save_records(&self, records: Vec<models::NewRecord>) -> Result<Vec<models::Record>> {
        // TODO: do we need to return updated rows?
        let mut key_to_rec = records
            .into_iter()
            .map(|f| ((f.source_record_id.clone(), f.source_id), f))
            .collect::<HashMap<(String, i32), models::NewRecord>>();
        for record in records::table
            .filter(
                records::source_record_id.eq_any(
                    key_to_rec
                        .keys()
                        .into_iter()
                        .map(|(sri, _)| sri.clone())
                        .collect::<Vec<String>>(),
                ),
            )
            .load_async::<models::Record>(&self.pool)
            .await?
        {
            let record_id = record.id;
            key_to_rec
                .remove(&(record.source_record_id, record.source_id))
                .map(|r| async move {
                    let update_result =
                        diesel::update(records::table.filter(records::id.eq(record_id)))
                            .set((
                                records::title.eq(r.title.clone()),
                                records::content.eq(r.content.clone()),
                                records::image.eq(r.image.clone()),
                            ))
                            .execute_async(&self.pool)
                            .await;
                    match update_result {
                        Ok(_) => {}
                        Err(e) => error!("{}", e),
                    }
                });
        }

        Ok(diesel::insert_into(records::table)
            .values(
                key_to_rec
                    .values()
                    .cloned()
                    .collect::<Vec<models::NewRecord>>(),
            )
            .on_conflict((records::source_record_id, records::source_id))
            .do_nothing()
            .get_results_async(&self.pool)
            .await?)
    }

    async fn set_source_scraped_now(&self, source: models::Source) -> Result<()> {
        update(sources::table.filter(sources::id.eq(source.id)))
            .set(sources::last_scrape_time.eq(now))
            .execute_async(&self.pool)
            .await?;
        Ok(())
    }

    async fn search_source(&self, query: &str) -> Result<Vec<models::Source>> {
        let like = format!("%{}%", query);
        let source = sources::table
            .filter(
                sources::origin.like(like.clone()).or(sources::external_link
                    .like(like.clone())
                    .or(sources::name.like(like.clone()))),
            )
            .get_results_async::<models::Source>(&self.pool)
            .await;
        match source {
            Err(te) => match &te {
                tokio_diesel::AsyncError::Error(de) => match de {
                    diesel::NotFound => Ok(vec![]),
                    _ => Err(te.into()),
                },
                _ => Err(te.into()),
            },
            Ok(s) => Ok(s),
        }
    }

    async fn get_sources_by_kind(&self, kind: String) -> Result<Vec<models::Source>> {
        Ok(sources::table
            .filter(sources::kind.eq(kind))
            .load_async::<models::Source>(&self.pool)
            .await?)
    }

    async fn get_sources_by_kind_for_scrape(
        &self,
        kind: String,
        check_secs_interval: &i32,
    ) -> Result<Vec<models::Source>> {
        Ok(sources::table
            .filter(
                sources::kind
                    .eq(kind)
                    .and(sources::last_scrape_time.le(now - check_secs_interval.second())),
            )
            .load_async::<models::Source>(&self.pool)
            .await?)
    }

    async fn save_sources(&self, sources: Vec<models::NewSource>) -> Result<Vec<models::Source>> {
        Ok(diesel::insert_into(sources::table)
            .values(sources)
            .on_conflict((sources::origin, sources::kind))
            .do_update()
            .set(sources::name.eq(excluded(sources::name)))
            .get_results_async::<models::Source>(&self.pool)
            .await?)
    }
}

impl From<tokio_diesel::AsyncError> for Error {
    fn from(err: tokio_diesel::AsyncError) -> Self {
        Self::DbError(err.to_string())
    }
}

impl From<&tokio_diesel::AsyncError> for Error {
    fn from(err: &tokio_diesel::AsyncError) -> Self {
        Self::DbError(err.to_string())
    }
}
