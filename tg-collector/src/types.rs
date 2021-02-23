use rust_tdlib::types::{
    Animation, Audio, Chat, File, PhotoSize, Poll, PollOption, Sticker, Supergroup,
    SupergroupFullInfo, Video, VideoNote,
};
use serde::Serialize;

#[derive(Debug)]
pub struct Channel {
    pub chat_id: i64,
    pub title: String,
    pub description: String,
    pub username: String,
}

impl Channel {
    pub fn convert(
        chat: &Chat,
        supergroup: &Supergroup,
        supergroup_full_info: &SupergroupFullInfo,
    ) -> Self {
        Channel {
            chat_id: chat.id(),
            title: chat.title().clone(),
            description: supergroup_full_info.description().clone(),
            username: supergroup.username().to_string(),
        }
    }
}

#[derive(Debug)]
pub enum TelegramUpdate {
    FileDownloadFinished(TelegramFile),
    Message(TelegramMessage),
}

#[derive(Debug)]
pub struct TelegramMessage {
    pub message_id: i64,
    pub chat_id: i64,
    pub date: Option<i64>,
    pub content: Option<String>,
    pub files: Option<Vec<TelegramFileWithMeta>>,
}

#[derive(Debug, Serialize)]
pub struct ImageMeta {
    pub width: i64,
    pub height: i64,
}

impl From<&PhotoSize> for ImageMeta {
    fn from(p: &PhotoSize) -> Self {
        Self {
            width: p.width(),
            height: p.height(),
        }
    }
}
impl From<&Sticker> for ImageMeta {
    fn from(s: &Sticker) -> Self {
        Self {
            width: s.width(),
            height: s.height(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct AnimationMeta {
    pub duration: i64,
    pub width: i64,
    pub height: i64,
    pub mime_type: String,
}

impl From<&Animation> for AnimationMeta {
    fn from(a: &Animation) -> Self {
        Self {
            duration: a.duration(),
            width: a.width(),
            height: a.height(),
            mime_type: a.mime_type().clone(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct VideoMeta {
    pub duration: i64,
    pub width: i64,
    pub height: i64,
}

impl From<&Video> for VideoMeta {
    fn from(video: &Video) -> Self {
        Self {
            duration: video.duration(),
            width: video.width(),
            height: video.height(),
        }
    }
}

impl From<&VideoNote> for VideoMeta {
    fn from(video: &VideoNote) -> Self {
        Self {
            duration: video.duration(),
            width: video.length(),
            height: video.length(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct AudioMeta {
    pub duration: i64,
    pub title: String,
    pub performer: String,
}

impl From<&Audio> for AudioMeta {
    fn from(audio: &Audio) -> Self {
        Self {
            duration: audio.duration(),
            title: audio.title().clone(),
            performer: audio.performer().clone(),
        }
    }
}
#[derive(Debug, Serialize)]
pub struct PollMeta {
    pub question: String,
    pub options: Vec<PollOptionMeta>,
    pub total_voter_count: i64,
}

impl From<&Poll> for PollMeta {
    fn from(p: &Poll) -> Self {
        Self {
            question: p.question().clone(),
            options: p.options().iter().map(|f| f.into()).collect(),
            total_voter_count: p.total_voter_count(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct PollOptionMeta {
    pub text: String,
    pub voter_count: i64,
    pub vote_percentage: i64,
}

impl From<&PollOption> for PollOptionMeta {
    fn from(po: &PollOption) -> Self {
        Self {
            text: po.text().clone(),
            voter_count: po.voter_count(),
            vote_percentage: po.vote_percentage(),
        }
    }
}

#[derive(Debug)]
pub enum FileType {
    Document,
    Audio(AudioMeta),
    Video(VideoMeta),
    Animation(AnimationMeta),
    Image(ImageMeta),
}

#[derive(Debug)]
pub struct TelegramFileWithMeta {
    pub path: FilePath,
    pub file_type: FileType,
    pub file_name: Option<String>,
}

#[derive(Debug)]
pub struct FilePath {
    pub local_path: Option<String>,
    pub remote_file: String,
    pub remote_id: String,
}

impl From<&File> for FilePath {
    fn from(file: &File) -> Self {
        Self {
            local_path: Some(file.local().path().as_str())
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string()),
            remote_file: file.id().to_string(),
            remote_id: file.remote().unique_id().clone(),
        }
    }
}

#[derive(Debug)]
pub struct TelegramFile {
    pub local_path: String,
    pub remote_file: String,
    pub remote_id: String,
}

#[derive(Debug)]
pub struct TelegramFileForRecord {
    pub file: TelegramFile,
    pub record_id: String,
}
