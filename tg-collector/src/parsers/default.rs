use super::TelegramDataParser;
use crate::result::{Error, Result};
use crate::tg_client::TgUpdate;
use crate::types::*;
use async_trait::async_trait;
use rust_tdlib::types::{FormattedText, MessageContent, TextEntity, TextEntityType};

#[derive(Clone, Debug)]
pub struct DefaultTelegramParser;

impl DefaultTelegramParser {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for DefaultTelegramParser {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl TelegramDataParser for DefaultTelegramParser {
    async fn parse_update(&self, tg_update: &TgUpdate) -> Result<Option<TelegramUpdate>> {
        Ok(match tg_update {
            TgUpdate::FileDownloaded(update_file) => {
                Some(TelegramUpdate::FileDownloadFinished(TelegramFile {
                    local_path: update_file.file().local().path().clone(),
                    remote_file: update_file.file().id().to_string(),
                    remote_id: update_file.file().remote().unique_id().to_string(),
                }))
            }
            TgUpdate::NewMessage(new_message) => {
                let (content, files) = self
                    .parse_message_content(new_message.message().content())
                    .await?;
                if content.is_none() && files.is_none() {
                    None
                } else {
                    Some(TelegramUpdate::Message(TelegramMessage {
                        message_id: new_message.message().id(),
                        chat_id: new_message.message().chat_id(),
                        date: Some(new_message.message().date()),
                        content,
                        files,
                    }))
                }
            }
            TgUpdate::MessageContent(message_content) => {
                let (content, files) = self
                    .parse_message_content(message_content.new_content())
                    .await?;

                if content.is_none() && files.is_none() {
                    None
                } else {
                    Some(TelegramUpdate::Message(TelegramMessage {
                        message_id: message_content.message_id(),
                        chat_id: message_content.chat_id(),
                        date: None,
                        content,
                        files,
                    }))
                }
            }
            TgUpdate::ChatPhoto(photo) => {
                return Err(Error::UpdateNotSupported("photo".to_string()))
            }
            TgUpdate::ChatTitle(title) => {
                return Err(Error::UpdateNotSupported("chat_title".to_string()))
            }
        })
    }

    async fn parse_message_content(
        &self,
        message: &MessageContent,
    ) -> Result<(Option<String>, Option<Vec<TelegramFileWithMeta>>)> {
        match message {
            MessageContent::MessageText(text) => {
                Ok((Some(self.parse_formatted_text(text.text())), None))
            }
            MessageContent::MessageAnimation(message_animation) => {
                let file = TelegramFileWithMeta {
                    path: message_animation.animation().animation().into(),
                    file_type: FileType::Animation(message_animation.animation().into()),
                    file_name: Some(message_animation.animation().file_name().clone()),
                };
                Ok((
                    Some(self.parse_formatted_text(message_animation.caption())),
                    Some(vec![file]),
                ))
            }
            MessageContent::MessageAudio(message_audio) => {
                let file = TelegramFileWithMeta {
                    path: message_audio.audio().audio().into(),
                    file_type: FileType::Audio(message_audio.audio().into()),
                    file_name: Some(message_audio.audio().file_name().clone()),
                };
                Ok((
                    Some(self.parse_formatted_text(message_audio.caption())),
                    Some(vec![file]),
                ))
            }
            MessageContent::MessageDocument(message_document) => {
                let file = TelegramFileWithMeta {
                    path: message_document.document().document().into(),
                    file_type: FileType::Document,
                    file_name: Some(message_document.document().file_name().clone()),
                };
                Ok((
                    Some(self.parse_formatted_text(message_document.caption())),
                    Some(vec![file]),
                ))
            }
            MessageContent::MessagePhoto(photo) => {
                let files = photo
                    .photo()
                    .sizes()
                    .iter()
                    .map(|s| TelegramFileWithMeta {
                        file_type: FileType::Image(s.into()),
                        path: s.photo().into(),
                        file_name: None,
                    })
                    .collect();
                Ok((
                    Some(self.parse_formatted_text(photo.caption())),
                    Some(files),
                ))
            }
            MessageContent::MessageVideo(message_video) => {
                let file = TelegramFileWithMeta {
                    path: message_video.video().video().into(),
                    file_type: FileType::Video(message_video.video().into()),
                    file_name: Some(message_video.video().file_name().clone()),
                };
                Ok((
                    Some(self.parse_formatted_text(message_video.caption())),
                    Some(vec![file]),
                ))
            }

            MessageContent::MessageChatChangePhoto(u) => {
                Err(Error::UpdateNotSupported("message_chat_change_photo".to_string()))
            }

            MessageContent::MessagePoll(u) => {
                Err(Error::UpdateNotSupported("message_poll".to_string()))
            }
            MessageContent::MessageChatChangeTitle(u) => {
                Err(Error::UpdateNotSupported("message_chat_change_title".to_string()))
            }
            MessageContent::MessageChatDeletePhoto(u) => {
                Err(Error::UpdateNotSupported("foo".to_string()))
            }
            MessageContent::MessageChatJoinByLink(u) => {
                Err(Error::UpdateNotSupported("foo".to_string()))
            }
            MessageContent::MessageChatUpgradeFrom(u) => {
                Err(Error::UpdateNotSupported("foo".to_string()))
            }
            MessageContent::MessageChatUpgradeTo(u) => {
                Err(Error::UpdateNotSupported("foo".to_string()))
            }
            MessageContent::MessageContact(u) => {
                Err(Error::UpdateNotSupported("foo".to_string()))
            }
            MessageContent::MessageContactRegistered(u) => {
                Err(Error::UpdateNotSupported("foo".to_string()))
            }
            MessageContent::MessageCustomServiceAction(u) => {
                Err(Error::UpdateNotSupported("foo".to_string()))
            }
            MessageContent::MessageExpiredPhoto(u) => {
                Err(Error::UpdateNotSupported("foo".to_string()))
            }
            MessageContent::MessageExpiredVideo(u) => {
                Err(Error::UpdateNotSupported("foo".to_string()))
            }
            MessageContent::MessageInvoice(u) => {
                Err(Error::UpdateNotSupported("foo".to_string()))
            }
            MessageContent::MessageLocation(u) => {
                Err(Error::UpdateNotSupported("foo".to_string()))
            }
            MessageContent::MessagePassportDataReceived(u) => {
                Err(Error::UpdateNotSupported("foo".to_string()))
            }
            MessageContent::MessageScreenshotTaken(u) => {
                Err(Error::UpdateNotSupported("foo".to_string()))
            }
            MessageContent::MessageSticker(message_sticker) => {
                let file = TelegramFileWithMeta {
                    path: message_sticker.sticker().sticker().into(),
                    file_type: FileType::Image(message_sticker.sticker().into()),
                    file_name: None,
                };
                Ok((None, Some(vec![file])))
            }
            MessageContent::MessageSupergroupChatCreate(u) => {
                Err(Error::UpdateNotSupported("foo".to_string()))
            }

            MessageContent::MessageVenue(u) => {
                Err(Error::UpdateNotSupported("foo".to_string()))
            }

            MessageContent::MessageVideoNote(message_video_note) => {
                let file = TelegramFileWithMeta {
                    path: message_video_note.video_note().video().into(),
                    file_type: FileType::Video(message_video_note.video_note().into()),
                    file_name: None,
                };
                Ok((None, Some(vec![file])))
            }
            MessageContent::MessageVoiceNote(u) => {
                Err(Error::UpdateNotSupported("foo".to_string()))
            }
            MessageContent::MessageWebsiteConnected(u) => {
                Err(Error::UpdateNotSupported("foo".to_string()))
            }

            MessageContent::_Default => Ok((None, None)),
            MessageContent::MessageBasicGroupChatCreate(_) => Ok((None, None)),
            MessageContent::MessageCall(_) => Ok((None, None)),
            MessageContent::MessageChatAddMembers(_) => Ok((None, None)),
            MessageContent::MessageChatDeleteMember(_) => Ok((None, None)),
            MessageContent::MessageChatSetTtl(_) => Ok((None, None)),
            MessageContent::MessageGame(_) => Ok((None, None)),
            MessageContent::MessageGameScore(_) => Ok((None, None)),
            MessageContent::MessagePassportDataSent(_) => Ok((None, None)),
            MessageContent::MessagePaymentSuccessful(_) => Ok((None, None)),
            MessageContent::MessagePaymentSuccessfulBot(_) => Ok((None, None)),
            MessageContent::MessagePinMessage(_) => Ok((None, None)),
            MessageContent::MessageUnsupported(_) => Ok((None, None)),
            MessageContent::MessageDice(_) => {Ok((None, None))}
            MessageContent::MessageProximityAlertTriggered(_) => {Ok((None, None))}
        }
    }

    fn parse_formatted_text(&self, formatted_text: &FormattedText) -> String {
        let mut entities_by_index = make_entities_stack(formatted_text.entities());
        let mut result_text = String::new();
        let mut current_entity = match entities_by_index.pop() {
            None => return formatted_text.text().clone(),
            Some(entity) => entity,
        };
        for (i, ch) in formatted_text.text().chars().enumerate() {
            if i == current_entity.0 {
                result_text = format!("{}{}{}", result_text, current_entity.1, ch);
                current_entity = match entities_by_index.pop() {
                    None => {
                        result_text = format!(
                            "{}{}",
                            result_text,
                            &formatted_text
                                .text()
                                .chars()
                                .skip(i + 1)
                                .take(formatted_text.text().len() - i)
                                .collect::<String>()
                        );
                        return result_text;
                    }
                    Some(entity) => entity,
                };
            } else {
                result_text.push(ch)
            }
        }
        result_text
    }
}

fn make_entities_stack(entities: &[TextEntity]) -> Vec<(usize, String)> {
    let mut stack = Vec::new();
    for entity in entities {
        let formatting = match entity.type_() {
            TextEntityType::Bold(_) => Some(("<b>".to_string(), "</b>".to_string())),
            TextEntityType::Code(_) => Some(("<code>".to_string(), "</code>".to_string())),
            TextEntityType::Hashtag(_) => Some(("#".to_string(), "".to_string())),
            TextEntityType::Italic(_) => Some(("<i>".to_string(), "</i>".to_string())),
            TextEntityType::PhoneNumber(_) => Some(("<phone>".to_string(), "</phone>".to_string())),
            TextEntityType::Pre(_) => Some(("<pre>".to_string(), "</pre>".to_string())),
            TextEntityType::PreCode(_) => {
                Some(("<pre><code>".to_string(), "</code></pre>".to_string()))
            }
            TextEntityType::Strikethrough(_) => {
                Some(("<strike>".to_string(), "</strike>".to_string()))
            }
            TextEntityType::TextUrl(u) => {
                let tag = format!(r#"<a href="{}">"#, u.url());
                Some((tag, "</a>".to_string()))
            }
            TextEntityType::Underline(_) => Some(("<u>".to_string(), "</u>".to_string())),
            TextEntityType::Url(_) => Some(("<a>".to_string(), "</a>".to_string())),
            TextEntityType::_Default => None,
            // TextEntityType::BankCardNumber(_) => None,
            TextEntityType::BotCommand(_) => None,
            TextEntityType::Cashtag(_) => None,
            TextEntityType::EmailAddress(_) => None,
            TextEntityType::Mention(_) => None,
            TextEntityType::MentionName(_) => None,
            TextEntityType::BankCardNumber(_) => None,
        };
        if let Some((start_tag, end_tag)) = formatting {
            stack.push((entity.offset() as usize, start_tag));
            stack.push(((entity.offset() + entity.length()) as usize, end_tag));
        }
    }
    stack.sort_by_key(|(i, _)| *i);
    stack.reverse();
    stack
}

#[cfg(test)]
mod tests {
    use crate::parsers::default::DefaultTelegramParser;
    use rust_tdlib::types::FormattedText;

    #[test]
    fn test_parse_formatted_text() {
        let p = DefaultTelegramParser::new();
        let tests = vec![
            (
                r#"{"@type":"formattedText","@extra":"","text":"Изображение из пятидесяти линий.\nНаткнулся на скрипт, который генерирует такие изображения вот тут.\nЛожите рядом со скриптом png изображение 750х750 в градациях серого, в исходнике меняете имя файла на ваше и запускаете исходник с помощью processing. Сгенерированное изображение будет лежать в том же каталоге.","entities":[{"@type":"textEntity","@extra":"","offset":91,"length":7,"type":{"@type":"textEntityTypeTextUrl","@extra":"","url":"https://gist.github.com/u-ndefine/8e4bc21be4275f87fefe7b2a68487161"}},{"@type":"textEntity","@extra":"","offset":239,"length":10,"type":{"@type":"textEntityTypeTextUrl","@extra":"","url":"https://processing.org/download/"}}]}"#,
                r#"Изображение из пятидесяти линий.
Наткнулся на скрипт, который генерирует такие изображения <a href="https://gist.github.com/u-ndefine/8e4bc21be4275f87fefe7b2a68487161">вот тут</a>.
Ложите рядом со скриптом png изображение 750х750 в градациях серого, в исходнике меняете имя файла на ваше и запускаете исходник с помощью <a href="https://processing.org/download/">processing</a>. Сгенерированное изображение будет лежать в том же каталоге."#,
            ),
            (
                r#"{"@type":"formattedText","@extra":"","text":"Напоминаем, что здесь у нас есть ещё и свой чат, где проходят «публичные» интервью, а в свободное время можно просто потрещать за жизнь \n\nЗаходи, тебе здесь рады)\n\nhttps://t.me/joinchat/IqlQqUGyZpI1-0Zu8ChAmA","entities":[]}"#,
                r#"Напоминаем, что здесь у нас есть ещё и свой чат, где проходят «публичные» интервью, а в свободное время можно просто потрещать за жизнь 

Заходи, тебе здесь рады)

https://t.me/joinchat/IqlQqUGyZpI1-0Zu8ChAmA"#,
            ),
        ];
        for (json_data, expected) in tests {
            let formatted_text = FormattedText::from_json(json_data).unwrap();
            let t = p.parse_formatted_text(&formatted_text);
            assert_eq!(t, expected);
        }
    }
}
