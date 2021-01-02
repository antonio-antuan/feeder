use chrono::NaiveDateTime;

#[derive(Debug, Serialize, Clone)]
pub struct Feed {
    pub image: Option<String>,
    pub link: String,
    pub kind: FeedKind,
    pub name: String,
    pub content: Vec<FeedItem>,
}

#[derive(Serialize, Debug, Clone)]
pub struct FeedItem {
    pub title: Option<String>,
    pub content: String,
    pub pub_date: NaiveDateTime,
    pub guid: String,
    pub image_link: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Copy, Clone, PartialEq)]
pub enum FeedKind {
    RSS,
    Atom,
    WP,
}
