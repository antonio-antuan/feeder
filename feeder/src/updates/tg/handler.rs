use crate::result::{Error, Result};
use crate::updates::SourceData;
use std::sync::Arc;
use tg_collector::parsers::TelegramDataParser;
use tg_collector::tg_client::{TgClient, TgUpdate};
use tokio::stream::StreamExt;
use tokio::sync::{mpsc, Mutex, RwLock};
use tokio::task::{spawn, JoinHandle};

/// Handler interacts with tdlib using `tg_collector` crate.
/// It initializes updates listener and pass all updates from `tg_collector` to specified sender

#[derive(Clone)]
pub struct Handler<P>
where
    P: TelegramDataParser + Send + Sync + Clone + 'static,
{
    sender: Arc<Mutex<mpsc::Sender<Result<SourceData>>>>,
    tg: Arc<RwLock<TgClient>>,
    orig_sender: mpsc::Sender<TgUpdate>,
    orig_receiver: Arc<Mutex<mpsc::Receiver<TgUpdate>>>,
    parser: P,
}

impl<P> Handler<P>
where
    P: TelegramDataParser + Send + Sync + Clone + 'static,
{
    /// Creates new Handler with specified
    pub fn new(
        sender: Arc<Mutex<mpsc::Sender<Result<SourceData>>>>,
        tg: Arc<RwLock<TgClient>>,
        parser: P,
    ) -> Self {
        // TODO: configure channel size
        let (orig_sender, orig_receiver) = mpsc::channel::<TgUpdate>(2000);
        Self {
            sender,
            tg,
            orig_sender,
            parser,
            orig_receiver: Arc::new(Mutex::new(orig_receiver)),
        }
    }

    pub async fn run(&mut self) {
        let mut guard = self.tg.write().await;
        guard.start().await;
        let recv = self.orig_receiver.clone();
        let sender = self.sender.clone();
        let parser = self.parser.clone();
        spawn(async move {
            loop {
                let update = recv.lock().await.recv().await;
                match &update {
                    None => return,
                    Some(update) => {
                        let parsed_update = match parser.parse_update(update).await {
                            Ok(Some(update)) => Ok(SourceData::Telegram(update)),
                            Err(e) => Err(Error::TgCollectorError(e)),

                            Ok(None) => continue,
                        };
                        let mut local_sender = sender.lock().await;

                        if let Err(err) = local_sender.send(parsed_update).await {
                            warn!("{}", err)
                        }
                    }
                }
            }
        });
    }
}
