use async_trait::async_trait;
use vk_collector::{
    client::VkClient,
    result::Result as VkResult,
    types::{Group, WallItem},
};

use super::{SourceData, SourceProvider, UpdatesHandler};
use crate::models;
use crate::result::{Error, Result};
use crate::storage::Storage;

use crate::updates::Source;
use chrono::NaiveDateTime;
use futures::future::join_all;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::mpsc::Sender;
use tokio::sync::{mpsc, Mutex};
use tokio::time;

// TODO: enum?
const VK: &str = "VK";

pub struct VkSource<S>
where
    S: Storage + Send + Sync + Clone + 'static,
{
    sleep_secs: u64,
    scrape_source_secs_interval: i32,
    client: Arc<VkClient>,
    storage: S,
}

impl<S> VkSource<S>
where
    S: Storage + Send + Sync + Clone + 'static,
{
    pub fn builder() -> VkSourceBuilder<S> {
        VkSourceBuilder::new()
    }
}

pub struct VkSourceBuilder<S>
where
    S: Storage + Send + Sync + Clone + 'static,
{
    sleep_secs: u64,
    scrape_source_secs_interval: i32,
    storage: Option<S>,
    token: Option<String>,
    // TODO: specify http client
}

impl<S> Default for VkSourceBuilder<S>
where
    S: Storage + Send + Sync + Clone + 'static,
{
    fn default() -> Self {
        Self::new()
    }
}
impl<S> VkSourceBuilder<S>
where
    S: Storage + Send + Sync + Clone + 'static,
{
    pub fn new() -> Self {
        Self {
            sleep_secs: 60,
            scrape_source_secs_interval: 60,
            storage: None,
            token: None,
        }
    }

    pub fn with_sleep_secs(mut self, sleep_secs: u64) -> Self {
        self.sleep_secs = sleep_secs;
        self
    }

    pub fn with_scrape_source_secs_interval(mut self, scrape_source_secs_interval: i32) -> Self {
        self.scrape_source_secs_interval = scrape_source_secs_interval;
        self
    }

    pub fn with_storage(mut self, storage: S) -> Self {
        self.storage = Some(storage);
        self
    }

    pub fn with_token(mut self, token: String) -> Self {
        self.token = Some(token);
        self
    }

    pub fn build(self) -> VkSource<S> {
        if self.storage.is_none() {
            panic!("storage not specified")
        }
        if self.token.is_none() {
            panic!("vk token not specified")
        }
        VkSource {
            sleep_secs: self.sleep_secs,
            scrape_source_secs_interval: self.scrape_source_secs_interval,
            client: Arc::new(VkClient::new(
                self.token.unwrap().as_str(),
                reqwest::Client::new(),
                3,
                1,
            )),
            storage: self.storage.unwrap(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct VkUpdate {
    id: i64,
    owner_id: i64,
    from_id: i64,
    date: i64,
    text: String,
}

impl From<WallItem> for VkUpdate {
    fn from(i: WallItem) -> Self {
        VkUpdate {
            id: i.id(),
            owner_id: i.owner_id(),
            from_id: i.from_id(),
            date: i.date(),
            text: i.text().to_string(),
        }
    }
}

#[async_trait]
impl<S> UpdatesHandler<VkUpdate> for VkSource<S>
where
    S: Storage + Send + Sync + Clone + 'static,
{
    async fn create_source(&self, updates: &VkUpdate) -> Result<models::Source> {
        let groups = self
            .client
            .get_groups_by_ids(vec![updates.owner_id.to_string()])
            .await?;
        if groups.is_empty() {
            return Err(Error::SourceNotFound);
        }

        Ok(self
            .storage
            .save_sources(groups.into_iter().map(models::NewSource::from).collect())
            .await?
            .pop()
            .unwrap())
    }

    async fn process_updates(&self, updates: &VkUpdate) -> Result<usize> {
        let source = match self
            .storage
            .get_exact_source(VK.to_string(), updates.owner_id.to_string())
            .await?
        {
            None => self.create_source(updates).await?,
            Some(s) => s,
        };
        let affected = self
            .storage
            .save_records(vec![models::NewRecord {
                title: None,
                source_record_id: updates.id.to_string(),
                source_id: source.id,
                content: updates.text.clone(),
                date: Some(NaiveDateTime::from_timestamp(updates.date, 0)),
                image: None,
            }])
            .await?;
        if affected.is_empty() {
            self.storage
                .set_record_external_link(
                    updates.id.to_string(),
                    source.id,
                    updates.owner_id.to_string(),
                )
                .await?;
        }
        self.storage.set_source_scraped_now(source).await?;
        Ok(affected.len())
    }
}

#[async_trait]
impl<S> SourceProvider for VkSource<S>
where
    S: Storage + Send + Sync + Clone + 'static,
{
    fn get_source(&self) -> Source {
        Source::Vk
    }

    async fn run(&self, updates_sender: Arc<Mutex<Sender<Result<SourceData>>>>) -> Result<()> {
        let (sources_sender, sources_receiver) = mpsc::channel(2000);

        let sleep_secs = self.sleep_secs;
        let scrape_source_secs_interval = self.scrape_source_secs_interval;
        let st = self.storage.clone();
        tokio::spawn(async move {
            sources_gen(st, scrape_source_secs_interval, sleep_secs, sources_sender).await
        });

        let cl = self.client.clone();
        let handler = Handler::new(updates_sender);
        tokio::spawn(async move { run_scrapper(cl.as_ref(), sources_receiver, handler).await });
        Ok(())
    }

    async fn search_source(&self, query: &str) -> Result<Vec<models::Source>> {
        let groups = self.client.search_group(query, 0, 20).await?;
        let mut sources = vec![];
        for gr in groups {
            let source: models::NewSource = gr.into();
            match self.storage.save_sources(vec![source]).await {
                Ok(s) => sources.extend(s),
                Err(e) => error!("{:?}", e),
            }
        }
        Ok(sources)
    }

    async fn synchronize(&self, secs_depth: i32) -> Result<()> {
        debug!("start syncing {:?}", self.get_source());
        let groups = self.client.get_my_groups(0, 1000).await?;
        let group_to_source: HashMap<i64, i32> = self
            .storage
            .save_sources(groups.iter().map(models::NewSource::from).collect())
            .await?
            .into_iter()
            .map(|s| (s.origin.parse().unwrap(), s.id))
            .collect();
        for group in groups {
            let wall_items = self.client.get_wall(group.id(), 0, 100).await?;
            self.storage
                .save_records(
                    wall_items
                        .into_iter()
                        .map(|wall| models::NewRecord {
                            title: None,
                            source_record_id: wall.id().to_string(),
                            source_id: *group_to_source.get(&wall.owner_id()).unwrap(),
                            content: wall.text().to_string(),
                            date: Some(NaiveDateTime::from_timestamp(wall.date(), 0)),
                            image: None,
                        })
                        .collect(),
                )
                .await?;
        }
        Ok(())
    }
}

impl<S> VkSource<S> where S: Storage + Send + Sync + Clone + 'static {}

async fn get_records_for_source(client: &VkClient, source_id: String, handler: &Handler) {
    let update = client.get_wall(source_id.parse().unwrap(), 0, 25).await;
    handler.process(update).await;
}

// TODO: generic scrapper. trait?
async fn run_scrapper(
    client: &VkClient,
    mut sources_receiver: mpsc::Receiver<Vec<String>>,
    handler: Handler,
) {
    while let Some(sources) = sources_receiver.recv().await {
        let mut tasks = vec![];
        for source in sources {
            tasks.push(get_records_for_source(client, source, &handler));
        }
        join_all(tasks).await;
    }
}

impl From<Group> for crate::models::NewSource {
    fn from(group: Group) -> crate::models::NewSource {
        crate::models::NewSource {
            name: group.name().to_string(),
            origin: group.id().to_string(),
            kind: VK.to_string(),
            image: None,
            external_link: group.screen_name().to_string(),
        }
    }
}

impl From<&Group> for crate::models::NewSource {
    fn from(group: &Group) -> crate::models::NewSource {
        crate::models::NewSource {
            name: group.name().to_string(),
            origin: group.id().to_string(),
            kind: VK.to_string(),
            image: None,
            external_link: group.screen_name().to_string(),
        }
    }
}

// TODO: generic generator
async fn sources_gen<S: Storage>(
    storage: S,
    source_check_period: i32,
    sleep_period: u64,
    sender: mpsc::Sender<Vec<String>>,
) {
    let sleep_period = time::Duration::from_secs(sleep_period);
    loop {
        match get_sources(&storage, &source_check_period).await {
            Ok(sources) => {
                debug!("found sources for scrape: {:?}", sources);
                if let Err(err) = sender.send(sources).await {
                    error!("{}", err)
                };
            }
            Err(e) => error!("{}", e),
        };

        debug!("send sources delayed for {:?}", sleep_period);
        tokio::time::sleep(sleep_period).await;
    }
}

async fn get_sources<S: Storage>(
    storage: &S,
    source_check_period_secs: &i32,
) -> Result<Vec<String>> {
    Ok(storage
        .get_sources_by_kind_for_scrape(VK.to_string(), source_check_period_secs)
        .await?
        .iter()
        .map(|r| r.origin.clone())
        .collect())
}

struct Handler {
    sender: Arc<Mutex<mpsc::Sender<Result<SourceData>>>>,
}

impl Handler {
    pub fn new(sender: Arc<Mutex<mpsc::Sender<Result<SourceData>>>>) -> Self {
        Self { sender }
    }

    async fn process(&self, result: VkResult<Vec<WallItem>>) {
        let send = |d| async {
            let local = self.sender.lock().await;
            if local.send(d).await.is_err() {
                error!("updates receiver dropped");
            }
        };

        match result {
            Ok(updates) => {
                for update in updates {
                    send(Ok(SourceData::Vk(VkUpdate::from(update)))).await
                }
            }
            Err(err) => send(Err(Error::VkCollectorError(err))).await,
        };
    }
}
