use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Serialize, Deserialize, Clone, Debug)]
pub struct RecordWithMeta {
    pub id: i32,
    pub title: Option<String>,
    pub guid: String,
    pub source_id: i32,
    pub content: String,
    pub date: NaiveDateTime,
    pub image: Option<String>,
    pub starred: Option<bool>,
    pub tags: Option<Vec<String>>,
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Clone, Debug)]
pub struct SourceWithMeta {
    pub id: i32,
    pub name: String,
    pub origin: String,
    pub kind: String,
    pub image: Option<String>,
    pub last_scrape_time: NaiveDateTime,
    pub external_link: String,
    pub folder_id: i32,
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Clone, Debug)]
pub struct User {
    pub id: i32,
    pub last_read_date: NaiveDateTime,
    pub token: Option<String>,
    pub login: String,
    pub password: String,
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Clone, Debug)]
pub struct UserFolder {
    pub id: i32,
    pub name: String,
    pub user_id: i32,
    pub parent_folder_id: Option<i32>,
}
