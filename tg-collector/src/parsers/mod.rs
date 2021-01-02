use crate::result::Result;
use crate::tg_client::TgUpdate;
use crate::types::*;
use async_trait::async_trait;
use rust_tdlib::types::{FormattedText, MessageContent};

#[async_trait]
pub trait TelegramDataParser {
    async fn parse_update(&self, tg_update: &TgUpdate) -> Result<Option<TelegramUpdate>>;
    async fn parse_message_content(
        &self,
        message: &MessageContent,
    ) -> Result<(Option<String>, Option<Vec<TelegramFileWithMeta>>)>;
    fn parse_formatted_text(&self, formatted_text: &FormattedText) -> String;
}

pub mod default;
pub use default::*;
