use crate::result;
use serde::Deserialize;
use tokio::sync::oneshot;

type ErrorCode = i16;
const TOO_MANY_REQUESTS: ErrorCode = 6;

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
    pub error_code: ErrorCode,
    pub error_msg: String,
}

impl VkError {
    pub fn is_too_many_requests(&self) -> bool {
        self.error_code == TOO_MANY_REQUESTS
    }
}

#[derive(Debug, Deserialize, Clone)]
pub(crate) enum VkResponse<I> {
    #[serde(rename(deserialize = "error"))]
    Error(VkError),
    #[serde(rename(deserialize = "response"))]
    Response(GetItemsResponse<I>),
}

#[derive(Debug, Clone)]
pub(crate) enum MethodResults {
    WallGet(Vec<WallItem>),
    GroupSearch(Vec<Group>),
}

pub(crate) struct WallGetParameters {
    owner_id: i64,
    offset: u8,
    limit: u8,
}

impl WallGetParameters {
    pub fn owner_id(&self) -> i64 {
        self.owner_id
    }
    pub fn offset(&self) -> u8 {
        self.offset
    }
    pub fn limit(&self) -> u8 {
        self.limit
    }
}

pub(crate) struct JobWallGet {
    parameters: WallGetParameters,
    results_sender: oneshot::Sender<result::Result<Vec<WallItem>>>,
}

impl JobWallGet {
    fn new(
        parameters: WallGetParameters,
        results_sender: oneshot::Sender<result::Result<Vec<WallItem>>>,
    ) -> Self {
        Self {
            parameters,
            results_sender,
        }
    }

    pub fn create(
        owner_id: i64,
        offset: u8,
        limit: u8,
    ) -> (Self, oneshot::Receiver<result::Result<Vec<WallItem>>>) {
        let (s, r) = oneshot::channel();
        let job = JobWallGet::new(
            WallGetParameters {
                owner_id,
                offset,
                limit,
            },
            s,
        );
        (job, r)
    }

    pub fn get_parameters(&self) -> String {
        format!(
            "owner_id={owner_id}&offset={offset}&limit={limit}",
            limit = self.parameters.limit,
            offset = self.parameters.offset,
            owner_id = self.parameters.owner_id
        )
    }

    pub fn set_result(self, result: result::Result<Vec<WallItem>>) {
        self.results_sender.send(result);
    }
}

pub(crate) struct GroupSearchParameters {
    q: String,
    offset: u8,
    limit: u8,
}

impl GroupSearchParameters {
    pub fn q(&self) -> &str {
        &self.q
    }
    pub fn offset(&self) -> u8 {
        self.offset
    }
    pub fn limit(&self) -> u8 {
        self.limit
    }
}

pub(crate) struct JobGroupSearch {
    parameters: GroupSearchParameters,
    results_sender: oneshot::Sender<result::Result<Vec<Group>>>,
}

impl JobGroupSearch {
    fn new(
        parameters: GroupSearchParameters,
        results_sender: oneshot::Sender<result::Result<Vec<Group>>>,
    ) -> Self {
        Self {
            parameters,
            results_sender,
        }
    }

    pub fn create(
        q: String,
        offset: u8,
        limit: u8,
    ) -> (Self, oneshot::Receiver<result::Result<Vec<Group>>>) {
        let (s, r) = oneshot::channel();
        let job = JobGroupSearch::new(GroupSearchParameters { q, offset, limit }, s);
        (job, r)
    }

    pub fn get_parameters(&self) -> String {
        format!(
            "q={q}&offset={offset}&limit={limit}",
            limit = self.parameters.limit,
            offset = self.parameters.offset,
            q = self.parameters.q
        )
    }

    pub fn set_result(self, result: result::Result<Vec<Group>>) {
        self.results_sender.send(result);
    }
}

pub(crate) enum Job {
    WallGet(JobWallGet),
    GroupSearch(JobGroupSearch),
}

impl Job {
    pub fn get_method(&self) -> &str {
        match self {
            Job::WallGet(_) => "wall.get",
            Job::GroupSearch(_) => "group.search",
        }
    }
}
