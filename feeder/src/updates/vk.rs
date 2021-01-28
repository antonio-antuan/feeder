use async_trait::async_trait;
use vk_collector::client::VkClient;
use vk_collector::types::{Group, WallItem};

use super::{SourceData, SourceProvider, UpdatesHandler};
use crate::models;
use crate::result::{Error, Result};
use crate::storage::Storage;

use crate::updates::Source;
use chrono::NaiveDateTime;
use futures::future::join_all;
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

    async fn run(&self, updates_sender: Arc<Mutex<Sender<Result<SourceData>>>>) {
        let (sources_sender, sources_receiver) = mpsc::channel(2000);

        let sleep_secs = self.sleep_secs;
        let scrape_source_secs_interval = self.scrape_source_secs_interval;
        let st = self.storage.clone();
        tokio::spawn(async move {
            sources_gen(st, scrape_source_secs_interval, sleep_secs, sources_sender).await
        });

        let cl = self.client.clone();
        tokio::spawn(async move { run_scrapper(cl.as_ref(), sources_receiver).await });
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
        unimplemented!()
    }
}

impl<S> VkSource<S> where S: Storage + Send + Sync + Clone + 'static {}

async fn get_records_for_source(client: &VkClient, source_id: String) -> Result<Vec<VkUpdate>> {
    Ok(client
        .get_wall(source_id.parse().unwrap(), 0, 25)
        .await?
        .into_iter()
        .map(VkUpdate::from)
        .collect())
}

// TODO: generic scrapper. trait?
async fn run_scrapper(client: &VkClient, mut sources_receiver: mpsc::Receiver<Vec<String>>) {
    while let Some(sources) = sources_receiver.recv().await {
        let mut tasks = vec![];
        for source in sources {
            tasks.push(get_records_for_source(client, source));
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

// TODO: generic generator
async fn sources_gen<S: Storage>(
    storage: S,
    source_check_period: i32,
    sleep_period: u64,
    mut sender: mpsc::Sender<Vec<String>>,
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
        tokio::time::delay_for(sleep_period).await;
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
