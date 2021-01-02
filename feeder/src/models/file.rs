use serde::{Deserialize, Serialize};

#[cfg(feature = "pg-storage")]
use {
    crate::storage::schema::files,
    diesel::{Insertable, Queryable},
};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(feature = "pg-storage", derive(Queryable))]
pub struct File {
    pub id: i32,
    pub record_id: i32,
    pub kind: String,
    pub local_path: Option<String>,
    pub remote_path: String,
    pub remote_id: Option<String>,
    pub file_name: Option<String>,
    #[serde(rename(serialize = "type", deserialize = "type"))]
    pub type_: String,
    pub meta: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "pg-storage", derive(Insertable))]
#[cfg_attr(feature = "pg-storage", table_name = "files")]
pub struct NewFile {
    pub record_id: i32,
    pub kind: String,
    pub local_path: Option<String>,
    pub remote_path: String,
    pub remote_id: Option<String>,
    pub file_name: Option<String>,
    pub type_: String,
    pub meta: Option<String>,
}
