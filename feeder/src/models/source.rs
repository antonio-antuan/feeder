use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Debug, Clone, Serialize, Deserialize)]
pub struct Source {
    pub id: i32,
    pub name: String,
    pub origin: String,
    pub kind: String,
    pub image: Option<String>,
    pub last_scrape_time: NaiveDateTime,
    pub external_link: String,
}

#[derive(sqlx::FromRow, Debug, Clone, Serialize, Deserialize)]
pub struct NewSource {
    pub name: String,
    pub origin: String,
    pub kind: String,
    pub image: Option<String>,
    pub external_link: String,
}
