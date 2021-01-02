use super::handler::Handler;
use crate::models;
use crate::result::{Error, Result};
use crate::storage::Storage;
use crate::updates::tg::TelegramSource;
use crate::updates::{Source, SourceData, SourceProvider};
use async_trait::async_trait;
use chrono::NaiveDateTime;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tg_collector::parsers::TelegramDataParser;
use tg_collector::tg_client::TgClient;
use tokio::stream::StreamExt;
use tokio::sync::{mpsc, Mutex};

#[async_trait]
impl<S, P> SourceProvider for TelegramSource<S, P>
where
    S: Storage + Send + Sync,
    P: TelegramDataParser + Send + Sync + Clone + 'static,
{
    fn get_source(&self) -> Source {
        Source::Telegram
    }

    async fn run(&self, updates_sender: Arc<Mutex<mpsc::Sender<Result<SourceData>>>>) {
        let mut tg_handler =
            Handler::new(updates_sender, self.collector.clone(), self.parser.clone());
        tg_handler.run().await;
    }

    async fn search_source(&self, query: &str) -> Result<Vec<models::Source>> {
        let channels = self
            .collector
            .read()
            .await
            .search_public_chats(query)
            .await?;
        let mut sources = vec![];
        for ch in channels {
            let source: models::NewSource = ch.into();
            match self.storage.save_sources(vec![source]).await {
                Ok(s) => sources.extend(s),
                Err(e) => error!("{:?}", e),
            }
        }
        Ok(sources)
    }

    async fn synchronize(&self, secs_depth: i32) -> Result<()> {
        let channels = self.collector.read().await.get_all_channels(1000).await?;

        let until = SystemTime::now().duration_since(UNIX_EPOCH).unwrap()
            - Duration::new(secs_depth as u64, 0);

        debug!("got {} channels to sync", channels.len());

        for channel in channels {
            debug!("going to sync {}", channel.title);

            let chat_id = channel.chat_id;
            let source: models::NewSource = channel.into();

            let source = self
                .storage
                .save_sources(vec![source])
                .await?
                .pop()
                .unwrap();

            let mut messages_stream = Box::pin(TgClient::get_chat_history_stream(
                self.collector.clone(),
                chat_id,
                until.as_secs() as i64,
            ));

            let mut parsed_records = vec![];
            let mut files_by_rec = HashMap::new();

            while let Some(message) = messages_stream.next().await {
                match message {
                    Ok(message) => {
                        let mut on_content = |c| {
                            let record = models::NewRecord {
                                title: None,
                                source_record_id: message.id().to_string(),
                                source_id: source.id,
                                content: c,
                                date: Some(NaiveDateTime::from_timestamp(message.date(), 0)),
                                image: None,
                            };
                            parsed_records.push(record);
                        };
                        let mut on_file = |f| {
                            files_by_rec.insert((message.id(), source.id), f);
                        };

                        match self.parser.parse_message_content(message.content()).await {
                            Ok((Some(c), Some(f))) => {
                                on_content(c);
                                on_file(f)
                            }
                            Ok((Some(c), None)) => on_content(c),
                            Ok((None, Some(f))) => on_file(f),
                            Ok((None, None)) => {}
                            Err(tg_collector::result::Error::UpdateNotSupported(_)) => continue,
                            Err(e) => return Err(Error::TgCollectorError(e)),
                        }
                    }
                    Err(e) => return Err(Error::TgCollectorError(e)),
                }
            }

            debug!("get {} records for {}", parsed_records.len(), source.name);

            let records = self.storage.save_records(parsed_records).await?;
            for rec in &records {
                let rec_files =
                    files_by_rec.get(&(rec.source_record_id.parse().unwrap(), rec.source_id));
                match rec_files {
                    None => {}
                    Some(f) => {
                        if let Err(e) = self.handle_new_files(f, rec.id).await {
                            error!("{:?}", e)
                        };
                    }
                }
            }
        }
        Ok(())
    }
}
