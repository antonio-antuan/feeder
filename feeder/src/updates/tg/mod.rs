pub use source::*;
pub use source_provider::*;
use std::sync::Arc;
use tg_collector::parsers;
use tg_collector::types::Channel;
pub use tg_collector::types::TelegramUpdate;
pub use updates_handler::*;

// module reads from tdlib stream and pass updates to common app stream
mod handler;
// telegram source struct and methods
mod source;
// SourceProvider trait implementation
mod source_provider;
mod updates_handler;

pub type CloneableBoxedParser = Arc<Box<dyn parsers::TelegramDataParser + Send + Sync>>;

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
