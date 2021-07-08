pub mod records;
pub mod sources;
pub mod users;

use crate::db::models::UserFolder;
use crate::result::{Error, Result};
use std::convert::{TryFrom, TryInto};

impl TryFrom<crate::db::models::User> for users::User {
    type Error = Error;

    fn try_from(user: crate::db::models::User) -> Result<users::User, Self::Error> {
        Ok(users::User {
            id: user.id,
            last_read_date: user.last_read_date.timestamp().try_into().map_err(|_e| {
                Error::InternalServerError(format!("cannot convert date: {}", user.last_read_date))
            })?,
            login: user.login,
            token: user.token.unwrap_or("".to_string()).clone(),
        })
    }
}

impl From<crate::db::models::RecordWithMeta> for records::RecordWithMeta {
    fn from(record: crate::db::models::RecordWithMeta) -> Self {
        Self {
            record: Some(records::Record {
                source_record_id: record.guid,
                content: record.content,
                date: record.date.timestamp(),
                id: record.id,
                title: record.title.unwrap_or_default(),
                source_id: record.source_id,
                image: record.image.unwrap_or_default(),
            }),
            starred: record.starred.map_or(false, |v| v),
            tags: record.tags.unwrap_or_default(),
        }
    }
}

impl From<crate::db::models::SourceWithMeta> for sources::SourceWithMeta {
    fn from(source: crate::db::models::SourceWithMeta) -> Self {
        Self {
            id: source.id,
            external_link: source.external_link,
            name: source.name,
            origin: source.origin,
            kind: source.kind,
            image: source.image.unwrap_or_default(),
            last_scrape_time: source.last_scrape_time.timestamp(),
            folder_id: source.folder_id,
        }
    }
}

// we can't use `impl From` because all types are external. Adapter is the simplest way.
pub fn adapt_source(source: feeder::models::Source) -> sources::Source {
    sources::Source {
        id: source.id,
        name: source.name,
        origin: source.origin,
        kind: source.kind,
        image: source.image.unwrap_or_default(),
        last_scrape_time: source.last_scrape_time.timestamp(),
        external_link: source.external_link,
    }
}

impl From<crate::db::models::UserFolder> for users::get_folders_response::Folder {
    fn from(folder: UserFolder) -> Self {
        Self {
            id: folder.id,
            name: folder.name,
            parent_folder_id: folder.parent_folder_id.unwrap_or(0),
        }
    }
}
