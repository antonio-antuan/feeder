use super::CloneableBoxedParser;
use crate::result::{Error, Result};
use crate::updates::SourceData;
use std::sync::Arc;
use tg_collector::tg_client::{TgClient, TgUpdate};
use tokio::sync::{mpsc, Mutex, RwLock};
use tokio::task::spawn;

/// Handler interacts with tdlib using `tg_collector` crate.
/// It initializes updates listener and pass all updates from `tg_collector` to specified sender

#[derive(Clone)]
pub struct Handler {
    sender: Arc<Mutex<mpsc::Sender<Result<SourceData>>>>,
    tg: Arc<RwLock<TgClient>>,
    orig_sender: mpsc::Sender<TgUpdate>,
    orig_receiver: Arc<Mutex<mpsc::Receiver<TgUpdate>>>,
    parser: CloneableBoxedParser,
}

impl Handler {
    /// Creates new Handler with specified
    pub fn new(
        sender: Arc<Mutex<mpsc::Sender<Result<SourceData>>>>,
        tg: Arc<RwLock<TgClient>>,
        parser: CloneableBoxedParser,
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

    pub async fn run(&mut self) -> Result<()> {
        let mut guard = self.tg.write().await;
        let handle = match guard.start().await {
            Ok(h) => h,
            Err(e) => return Err(e.into()),
        };
        let recv = self.orig_receiver.clone();
        let sender = self.sender.clone();
        let parser = self.parser.clone();
        spawn(async move {
            tokio::select! {
                h = handle => info!("telegram client closed: {:?}", h),
                _ = async {
                    loop {
                        while let Some(update) = recv.lock().await.recv().await {
                            let parsed_update = match parser.parse_update(&update).await {
                                Ok(Some(update)) => Ok(SourceData::Telegram(update)),
                                Err(e) => Err(Error::TgCollectorError(e)),

                                Ok(None) => continue,
                            };
                            let local_sender = sender.lock().await;

                            if let Err(err) = local_sender.send(parsed_update).await {
                                warn!("{}", err)
                            }
                        };
                    }
                } => {info!("updates loop closed")}
            };
        });
        Ok(())
    }
}
