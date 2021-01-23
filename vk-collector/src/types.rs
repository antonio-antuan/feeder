use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct WallItem {
    id: i64,
    owner_id: i64,
    from_id: i64,
    date: u64,
    text: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Group {
    id: i64,
    name: String,
    screen_name: String,
}


#[derive(Debug, Deserialize, Clone)]
pub struct GetItemsResponse<I> {
    pub count: u32,
    pub items: Vec<I>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct VkError {
    pub error_code: i16,
    pub error_msg: String,
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) enum VkResponse<I> {
    #[serde(rename(deserialize = "error"))]
    Error(VkError),
    #[serde(rename(deserialize = "response"))]
    Response(GetItemsResponse<I>),
}