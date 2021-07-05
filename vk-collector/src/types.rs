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
    date: i64,
    text: String,
}

impl WallItem {
    pub fn id(&self) -> i64 {
        self.id
    }
    pub fn owner_id(&self) -> i64 {
        self.owner_id
    }
    pub fn from_id(&self) -> i64 {
        self.from_id
    }
    pub fn date(&self) -> i64 {
        self.date
    }
    pub fn text(&self) -> &str {
        &self.text
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Group {
    id: i64,
    name: String,
    screen_name: String,
}

impl Group {
    pub fn id(&self) -> i64 {
        self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn screen_name(&self) -> &str {
        &self.screen_name
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct ItemsWithCountResponse<I> {
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
pub(crate) enum VkResponse<T> {
    #[serde(rename(deserialize = "error"))]
    Error(VkError),
    #[serde(rename(deserialize = "response"))]
    ResponseWithCount(ItemsWithCountResponse<T>),
    ResponseArray(Vec<T>),
}

#[derive(Debug, Clone)]
pub(crate) enum MethodResults {
    WallGet(Vec<WallItem>),
    GroupSearch(Vec<Group>),
    GroupsGetById(Vec<Group>),
}

pub(crate) struct GroupsGetParameters {
    user_id: Option<u64>,
    extended: Option<bool>,
    filter: Option<String>,
    fields: Option<String>,
    offset: u32,
    count: u16,
}

pub(crate) struct JobGroupsGet {
    parameters: GroupsGetParameters,
    results_sender: oneshot::Sender<result::Result<Vec<Group>>>,
}

impl JobGroupsGet {
    fn new(
        parameters: GroupsGetParameters,
        results_sender: oneshot::Sender<result::Result<Vec<Group>>>,
    ) -> Self {
        Self {
            parameters,
            results_sender,
        }
    }

    pub fn create(
        user_id: Option<u64>,
        extended: Option<bool>,
        offset: u32,
        count: u16,
    ) -> (Self, oneshot::Receiver<result::Result<Vec<Group>>>) {
        let (s, r) = oneshot::channel();
        let job = JobGroupsGet::new(
            GroupsGetParameters {
                user_id,
                extended,
                offset,
                count,
                filter: None,
                fields: None,
            },
            s,
        );
        (job, r)
    }

    pub fn get_parameters(&self) -> String {
        let mut p = format!(
            "offset={offset}&count={count}",
            offset = &self.parameters.offset,
            count = &self.parameters.count
        );
        if let Some(u) = &self.parameters.user_id {
            p = format!("{}&user_id={u}", p, u = u);
        }

        if let Some(e) = self.parameters.extended {
            p = format!("{}&extended={e}", p, e = e as u8); // flag, we need to convert bool to 0 or 1
        }

        if let Some(f) = &self.parameters.filter {
            p = format!("{}&filter={f}", p, f = f);
        }

        if let Some(f) = &self.parameters.fields {
            p = format!("{}&fields={f}", p, f = f);
        }

        p
    }

    pub fn set_result(self, result: result::Result<Vec<Group>>) {
        if let Err(_) = self.results_sender.send(result) {
            log::error!("channel closed");
        }
    }
}
pub(crate) struct GroupsGetByIdParameters {
    group_ids: String,
    // group_id: String,
    fields: Option<String>,
}

pub(crate) struct JobGroupsGetById {
    parameters: GroupsGetByIdParameters,
    results_sender: oneshot::Sender<result::Result<Vec<Group>>>,
}

impl JobGroupsGetById {
    fn new(
        parameters: GroupsGetByIdParameters,
        results_sender: oneshot::Sender<result::Result<Vec<Group>>>,
    ) -> Self {
        Self {
            parameters,
            results_sender,
        }
    }

    pub fn create(
        group_ids: String,
        fields: Option<String>,
    ) -> (Self, oneshot::Receiver<result::Result<Vec<Group>>>) {
        let (s, r) = oneshot::channel();
        let job = JobGroupsGetById::new(GroupsGetByIdParameters { group_ids, fields }, s);
        (job, r)
    }

    pub fn get_parameters(&self) -> String {
        let mut r = format!(
            "group_ids={group_ids}",
            group_ids = &self.parameters.group_ids,
        );
        if let Some(f) = &self.parameters.fields {
            r = format!("{}&fields={fields}", r, fields = f)
        }
        r
    }

    pub fn set_result(self, result: result::Result<Vec<Group>>) {
        self.results_sender.send(result);
    }
}

pub(crate) struct WallGetParameters {
    owner_id: i64,
    offset: u8,
    limit: u8,
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
            limit = &self.parameters.limit,
            offset = &self.parameters.offset,
            owner_id = &self.parameters.owner_id
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
            limit = &self.parameters.limit,
            offset = &self.parameters.offset,
            q = &self.parameters.q
        )
    }

    pub fn set_result(self, result: result::Result<Vec<Group>>) {
        self.results_sender.send(result);
    }
}

pub(crate) enum Job {
    WallGet(JobWallGet),
    GroupSearch(JobGroupSearch),
    GroupsGetById(JobGroupsGetById),
    GroupsGet(JobGroupsGet),
}

impl Job {
    pub fn get_method(&self) -> &str {
        match self {
            Job::WallGet(_) => "wall.get",
            Job::GroupSearch(_) => "groups.search",
            Job::GroupsGetById(_) => "groups.getById",
            Job::GroupsGet(_) => "groups.get",
        }
    }
}
