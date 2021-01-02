// module reads from tdlib stream and pass updates to common app stream
mod handler;
// telegram source struct and methods
mod source;
// SourceProvider trait implementation
mod source_provider;
// UpdatesHandler trait implementation
mod updates_handler;

pub use source::*;
pub use source_provider::*;
use tg_collector::types::Channel;
pub use updates_handler::*;

impl From<Channel> for crate::models::NewSource {
    fn from(channel: Channel) -> crate::models::NewSource {
        crate::models::NewSource {
            name: channel.title,
            origin: channel.chat_id.to_string(),
            kind: TELEGRAM.to_string(),
            image: None,
            external_link: channel.username,
        }
    }
}

pub use tg_collector::types::TelegramUpdate;
