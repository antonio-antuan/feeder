use crate::throttler::{Throttler, Worker};
use crate::types::{Group, JobGroupSearch, JobWallGet, WallItem};
use crate::{
    result,
    types::{Job, VkResponse},
};
use async_trait::async_trait;
use reqwest::Client;
use serde::de::DeserializeOwned;
use std::sync::Arc;
use tokio::{sync, task::JoinHandle};

const BASE_URL: &str = "https://api.vk.com";
const API_VERSION: &str = "5.21";
const MAX_RPS: usize = 3;

#[derive(Debug, Clone)]
pub struct VkClient {
    runner: Arc<JoinHandle<()>>,
    jobs_sender: sync::mpsc::Sender<Job>,
}

impl VkClient {
    pub fn new(token: &str, client: Client, max_tasks_per_tick: usize, secs_tick: u64) -> Self {
        let worker = JobsWorker::new(client, token.to_string(), 3);
        let mut throttler = Throttler::new(tokio::time::Duration::from_secs(secs_tick), Arc::new(worker));
        let handle_run = throttler.run(max_tasks_per_tick);

        let (s, mut r) = sync::mpsc::channel(100);
        let handle_push = tokio::spawn(async move {
            if let Some(job) = r.recv().await {
                throttler.push(job).await;
            };
        });
        let handle = tokio::spawn(async move {
            tokio::select! {
                _ = handle_push => {}
                _ = handle_run => {}
            }
        });
        Self {
            runner: Arc::new(handle),
            jobs_sender: s,
        }
    }

    pub async fn get_wall(
        &mut self,
        owner_id: i64,
        offset: u8,
        limit: u8,
    ) -> result::Result<Vec<WallItem>>
where {
        let (job, res) = JobWallGet::create(owner_id, offset, limit);
        self.jobs_sender.send(Job::WallGet(job)).await?;
        res.await?
    }

    pub async fn search_group(
        &mut self,
        q: &str,
        offset: u8,
        limit: u8,
    ) -> result::Result<Vec<Group>> {
        let (job, res) = JobGroupSearch::create(q.to_string(), offset, limit);
        self.jobs_sender.send(Job::GroupSearch(job)).await?;
        res.await?
    }
}

struct JobsWorker {
    client: Client,
    token: String,
    max_tries: u8,
}

impl JobsWorker {
    pub fn new(client: Client, token: String, max_tries: u8) -> Self {
        Self {
            client,
            token,
            max_tries,
        }
    }
}

#[async_trait]
impl Worker<Job> for Arc<JobsWorker> {
    async fn call(&self, job: Job) {
        let params = match &job {
            Job::WallGet(wall_get) => wall_get.get_parameters(),
            Job::GroupSearch(group_search) => group_search.get_parameters(),
        };
        let url = format!(
            "{base_url}/method/{method}/?\
                access_token={token}&\
                v={v}&{params}",
            base_url = BASE_URL,
            token = self.token,
            method = job.get_method(),
            v = API_VERSION,
            params = params
        );

        match job {
            Job::WallGet(wg) => {
                let r = http_call_with_retry(&self.client, url.as_str(), self.max_tries).await;
                wg.set_result(r)
            }
            Job::GroupSearch(gs) => {
                let r = http_call_with_retry(&self.client, url.as_str(), self.max_tries).await;
                gs.set_result(r)
            }
        };
    }
}

async fn http_call_with_retry<T>(
    client: &Client,
    url: &str,
    max_tries: u8,
) -> result::Result<Vec<T>>
where
    T: DeserializeOwned,
{
    let mut try_num = 0;

    let call = || async {
        match client.get(url).send().await {
            Ok(r) => Ok(r.json().await?),
            Err(e) => Err(e.into()),
        }
    };
    let mut res: result::Result<VkResponse<T>> = call().await;

    while try_num < max_tries {
        match &res {
            Err(result::Error::VkError(err)) => {
                if err.is_too_many_requests() {
                    if try_num <= max_tries {
                        try_num += 1;
                    }
                }
            }
            _ => break,
        }
        res = call().await;
    }
    let res = match res {
        Ok(vk_response) => match vk_response {
            VkResponse::Error(vk_err) => Err(result::Error::VkError(vk_err)),
            VkResponse::Response(vk_resp) => Ok(vk_resp.items),
        },
        Err(err) => Err(err),
    };
    res
}
