use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[cfg(feature = "pg-storage")]
use {
    crate::storage::schema::records,
    diesel::{Insertable, Queryable},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "pg-storage", derive(Queryable))]
pub struct Record {
    pub id: i32,
    pub title: Option<String>,
    pub source_record_id: String,
    pub source_id: i32,
    pub content: String,
    pub date: NaiveDateTime,
    pub image: Option<String>,
    pub external_link: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "pg-storage", derive(Insertable))]
#[cfg_attr(feature = "pg-storage", table_name = "records")]
pub struct NewRecord {
    pub title: Option<String>,
    // TODO: add date, modify date (for app, not for source)
    pub source_record_id: String,
    pub source_id: i32,
    pub content: String,
    pub date: Option<NaiveDateTime>,
    pub image: Option<String>,
}
