pub mod users {
    use crate::result::{Error, Result};
    use std::convert::{TryFrom, TryInto};

    tonic::include_proto!("users");

    impl TryFrom<crate::db::models::User> for User {
        type Error = Error;

        fn try_from(user: crate::db::models::User) -> Result<User, Self::Error> {
            Ok(User {
                id: user.id,
                last_read_date: user.last_read_date.timestamp().try_into().map_err(|_e| {
                    Error::InternalServerError(format!(
                        "cannot convert date: {}",
                        user.last_read_date
                    ))
                })?,
                login: user.login,
                token: "".to_string(),
            })
        }
    }
}

pub mod records {
    tonic::include_proto!("records");

    impl From<crate::db::models::RecordWithMeta> for RecordWithMeta {
        fn from(record: crate::db::models::RecordWithMeta) -> Self {
            Self {
                record: Some(Record {
                    source_record_id: record.guid,
                    content: record.content,
                    date: record.date.timestamp(),
                    id: record.id,
                    title: record.title.unwrap_or_default(),
                    source_id: record.source_id,
                    image: record.image.unwrap_or_default(),
                }),
                starred: record.starred.map_or(false, |v| v),
                tags: record.tags.into_iter().filter_map(|v| v).collect(),
            }
        }
    }
}

pub mod sources {
    tonic::include_proto!("sources");

    impl From<crate::db::models::SourceWithMeta> for SourceWithMeta {
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

    pub fn adapt_source(source: feeder::models::Source) -> Source {
        Source {
            id: source.id,
            name: source.name,
            origin: source.origin,
            kind: source.kind,
            image: source.image.unwrap_or_default(),
            last_scrape_time: source.last_scrape_time.timestamp(),
            external_link: source.external_link,
        }
    }
}
