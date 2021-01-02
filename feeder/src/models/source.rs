use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[cfg(feature = "pg-storage")]
use {
    crate::storage::schema::sources,
    diesel::{Insertable, Queryable},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "pg-storage", derive(Queryable))]
pub struct Source {
    pub id: i32,
    pub name: String,
    pub origin: String,
    pub kind: String,
    pub image: Option<String>,
    pub last_scrape_time: NaiveDateTime,
    pub external_link: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "pg-storage", derive(Insertable))]
#[cfg_attr(feature = "pg-storage", table_name = "sources")]
pub struct NewSource {
    pub name: String,
    pub origin: String,
    pub kind: String,
    pub image: Option<String>,
    pub external_link: String,
}
