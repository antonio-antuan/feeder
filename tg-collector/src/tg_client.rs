#![allow(clippy::mutex_atomic)]
use async_trait::async_trait;
use rust_tdlib::client::api::{Api, RawApi};
use rust_tdlib::client::client::{Client, ClientState};
use rust_tdlib::types::{AuthorizationStateWaitCode, AuthorizationStateWaitEncryptionKey, AuthorizationStateWaitOtherDeviceConfirmation, AuthorizationStateWaitPassword, AuthorizationStateWaitPhoneNumber, AuthorizationStateWaitRegistration, Chat, ChatType, Chats, Close, DownloadFile, File, GetChat, GetChatHistory, GetChats, GetMessageLink, GetSupergroup, GetSupergroupFullInfo, MessageLink, JoinChat, Message, Messages, Ok, SearchPublicChats, Supergroup, SupergroupFullInfo, Update, TdlibParameters, UpdateChatPhoto, UpdateChatTitle, UpdateFile, UpdateMessageContent, UpdateNewMessage};
use std::io;
use std::sync::{Arc, Mutex};

use crate::result::{Error, Result};
use crate::traits;
use crate::types;
use futures::future::join_all;
use futures::{Stream, StreamExt, TryStreamExt};
use rust_tdlib::client::{AuthStateHandler, ClientBuilder};
use rust_tdlib::errors::RTDResult;
use std::collections::VecDeque;
use std::time::Duration;
use tokio::sync::{mpsc, RwLock};
use tokio::task::JoinHandle;
use tokio::sync::mpsc::Sender;

pub type ApiId = i64;


#[derive(Debug)]
pub struct Config<'a> {
    pub max_download_queue_size: usize,
    pub log_download_state_secs_interval: u64,
    pub log_verbosity_level: i32,
    pub encryption_key: &'a str,
    pub database_directory: &'a str,
    pub api_id: ApiId,
    pub api_hash: &'a str,
    pub phone_number: &'a str,
}


#[derive(Clone, Debug)]
struct AuthHandler {
    encryption_key: String,
    phone_number: String,
}

impl AuthHandler {
    pub fn new(encryption_key: &str, phone_number: &str) -> Self {
        Self {
            encryption_key: encryption_key.to_string(),
            phone_number: phone_number.to_string(),
        }
    }

    fn wait_input() -> String {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => input.trim().to_string(),
            Err(e) => panic!("Can not get input value: {:?}", e),
        }
    }
}

#[async_trait]
impl AuthStateHandler for AuthHandler {
    async fn handle_other_device_confirmation(
        &self,
        _: &AuthorizationStateWaitOtherDeviceConfirmation,
    ) {
        panic!("other device confirmation not supported")
    }

    async fn handle_wait_code(&self, _: &AuthorizationStateWaitCode) -> String {
        eprintln!("wait for auth code");
        AuthHandler::wait_input()
    }

    async fn handle_encryption_key(
        &self,
        _: &AuthorizationStateWaitEncryptionKey,
    ) -> String {
        self.encryption_key.to_string()
    }

    async fn handle_wait_password(&self, _: &AuthorizationStateWaitPassword) -> String {
        panic!("password not supported")
    }

    async fn handle_wait_phone_number(
        &self,
        _: &AuthorizationStateWaitPhoneNumber,
    ) -> String {
        self.phone_number.to_string()
    }

    async fn handle_wait_registration(
        &self,
        _: &AuthorizationStateWaitRegistration,
    ) -> (String, String) {
        panic!("registration not supported")
    }
}

#[async_trait]
impl traits::TelegramClientTrait for Client<AuthHandler, RawApi> {
    async fn start(&mut self) -> Result<JoinHandle<ClientState>> {
        Ok(self.start().await?)
    }

    fn set_updates_sender(&mut self, updates_sender: Sender<TdType>) -> Result<()> {
        Ok(self.set_updates_sender(updates_sender)?)
    }
}

#[async_trait]
impl traits::TelegramAsyncApi for Api<RawApi> {
    async fn download_file(&self, download_file: DownloadFile) -> RTDResult<File> {
        self.download_file(download_file).await
    }

    async fn close(&self, close: Close) -> RTDResult<Ok> {
        self.close(close).await
    }

    async fn get_chat(&self, get_chat: GetChat) -> RTDResult<Chat> {
        self.get_chat(get_chat).await
    }

    async fn get_chats(&self, get_chats: GetChats) -> RTDResult<Chats> {
        self.get_chats(get_chats).await
    }

    async fn get_chat_history(&self, get_chat_history: GetChatHistory) -> RTDResult<Messages> {
        self.get_chat_history(get_chat_history).await
    }

    async fn get_message_link(&self, get_message_link: GetMessageLink) -> RTDResult<HttpUrl> {
        self.get_message_link(get_message_link).await
    }

    async fn search_public_chats(
        &self,
        search_public_chats: SearchPublicChats,
    ) -> RTDResult<Chats> {
        self.search_public_chats(search_public_chats).await
    }

    async fn join_chat(&self, join_chat: JoinChat) -> RTDResult<Ok> {
        self.join_chat(join_chat).await
    }

    async fn get_supergroup_full_info(
        &self,
        get_supergroup_full_info: GetSupergroupFullInfo,
    ) -> RTDResult<SupergroupFullInfo> {
        self.get_supergroup_full_info(get_supergroup_full_info)
            .await
    }

    async fn get_supergroup(&self, get_supergroup: GetSupergroup) -> RTDResult<Supergroup> {
        self.get_supergroup(get_supergroup).await
    }
}

#[derive(Debug, Default)]
struct DownloadQueue {
    queue_size: usize,
    queue: VecDeque<i64>,
    in_progress: Vec<i64>,
}

impl DownloadQueue {
    pub fn new(queue_size: usize) -> Self {
        Self {
            queue_size,
            ..Default::default()
        }
    }

    pub fn log_state(&self) {
        debug!("download queue state: {:?}", self);
    }

    pub fn is_in_progress(&self, obj: &i64) -> bool {
        self.in_progress.contains(&obj)
    }

    pub fn may_be_download(&mut self, obj: i64) -> bool {
        self.log_state();
        if self.in_progress.len() >= self.queue_size {
            self.queue.push_back(obj);
            false
        } else {
            self.in_progress.push(obj);
            true
        }
    }

    pub fn mark_as_done_and_get_new(&mut self, obj: &i64) -> Option<i64> {
        self.log_state();
        self.in_progress.retain(|x| x != obj);
        let new = self.queue.pop_front();
        match new {
            Some(n) => {
                self.in_progress.push(n);
                Some(n)
            }
            None => None,
        }
    }
}

#[derive(Debug)]
pub struct TgClientBuilder {
    max_download_queue_size: usize,
    log_download_state_secs_interval: u64,
    log_verbosity_level: i32,
    encryption_key: Option<String>,
    database_directory: String,
    api_id: Option<ApiId>,
    api_hash: Option<String>,
    phone_number: Option<String>,
}

impl TgClientBuilder {
    pub fn new() -> Self {
        Self {
            max_download_queue_size: 1,
            log_download_state_secs_interval: 10,
            log_verbosity_level: 0,
            encryption_key: None,
            database_directory: "tdlib".to_string(),
            api_id: None,
            api_hash: None,
            phone_number: None,
        }
    }

    pub fn with_max_download_queue_size(mut self, max_download_queue_size: usize) -> Self {
        self.max_download_queue_size = max_download_queue_size;
        self
    }

    pub fn with_log_download_state_secs_interval(
        mut self,
        log_download_state_secs_interval: u64,
    ) -> Self {
        self.log_download_state_secs_interval = log_download_state_secs_interval;
        self
    }

    pub fn with_log_verbosity_level(mut self, log_verbosity_level: i32) -> Self {
        self.log_verbosity_level = log_verbosity_level;
        self
    }

    pub fn with_encryption_key(mut self, encryption_key: String) -> Self {
        self.encryption_key = Some(encryption_key);
        self
    }

    pub fn with_database_directory(mut self, database_directory: String) -> Self {
        self.database_directory = database_directory;
        self
    }

    pub fn with_api_id(mut self, api_id: ApiId) -> Self {
        self.api_id = Some(api_id);
        self
    }

    pub fn with_api_hash(mut self, api_hash: String) -> Self {
        self.api_hash = Some(api_hash);
        self
    }

    pub fn with_phone_number(mut self, phone_number: String) -> Self {
        self.phone_number = Some(phone_number);
        self
    }

    pub fn build(self) -> Result<TgClient> {
        let encryption_key = match self.encryption_key {
            None => return Err(Error::Common("encryption_key not set".to_string())),
            Some(e) => e,
        };

        let phone_number = match self.phone_number {
            None => return Err(Error::Common("phone_number not set".to_string())),
            Some(p) => p,
        };

        let api_hash = match self.api_hash {
            None => return Err(Error::Common("api_hash not set".to_string())),
            Some(a) => a,
        };

        let api_id = match self.api_id {
            None => return Err(Error::Common("api_id not set".to_string())),
            Some(a) => a,
        };

        let cfg = Config {
            api_id,
            max_download_queue_size: self.max_download_queue_size,
            log_download_state_secs_interval: self.log_download_state_secs_interval,
            log_verbosity_level: self.log_verbosity_level,
            encryption_key: encryption_key.as_str(),
            database_directory: self.database_directory.as_str(),
            api_hash: api_hash.as_str(),
            phone_number: phone_number.as_str(),
        };
        Ok(TgClient::new(&cfg))
    }
}

#[derive(Clone)]
pub struct TgClient {
    client: Box<dyn traits::TelegramClientTrait>,
    api: Box<dyn traits::TelegramAsyncApi>,
    download_queue: Arc<Mutex<DownloadQueue>>,
}

impl TgClient {
    pub fn builder() -> TgClientBuilder {
        TgClientBuilder::new()
    }

    pub(self) fn new(config: &Config) -> Self {
        let tdlib_parameters = TdlibParameters::builder()
            .database_directory(&config.database_directory)
            .use_test_dc(false)
            .api_id(config.api_id)
            .api_hash(&config.api_hash)
            .system_language_code("en")
            .device_model("Desktop")
            .system_version("Unknown")
            .application_version(env!("CARGO_PKG_VERSION"))
            .enable_storage_optimizer(true)
            .build();
        let worker = Worker::builder().with_auth_state_handler(
            AuthHandler::new(config.encryption_key, config.phone_number)
        ).build().unwrap();
        let client = Client::builder()
            .with_tdlib_parameters(tdlib_parameters)
            .build()
            .unwrap();
        let api = Box::new(client.api().clone());
        let download_queue = Arc::new(Mutex::new(DownloadQueue::new(
            config.max_download_queue_size,
        )));

        if config.log_download_state_secs_interval != 0 {
            let q_log = download_queue.clone();
            let sleep = Duration::from_secs(config.log_download_state_secs_interval);
            tokio::spawn(async move {
                loop {
                    q_log.lock().unwrap().log_state();
                    tokio::time::delay_for(sleep).await;
                }
            });
        }
        let tg = TgClient {
            client: Box::new(client),
            api,
            download_queue,
        };
        tg
    }

    pub fn start_listen_updates(
        &mut self,
        updates_sender: mpsc::Sender<TgUpdate>,
    ) -> Result<()> {
        let (sx, mut rx) = mpsc::channel::<Update>(100);
        self.client.set_updates_sender(sx)?;

        let download_queue = self.download_queue.clone();
        let api = self.api.clone();
        let mut sender = updates_sender.clone();

        tokio::spawn(async move {
            while let Some(message) = rx.recv().await {
                match message {
                    TdType::UpdateNewMessage(new_message) => {
                        if let Err(err) = sender.send(TgUpdate::NewMessage(new_message)).await {
                            warn!("{}", err);
                        };
                    }
                    TdType::UpdateMessageContent(message_content) => {
                        if let Err(err) = sender
                            .send(TgUpdate::MessageContent(message_content))
                            .await
                        {
                            warn!("{}", err);
                        };
                    }
                    TdType::UpdateChatPhoto(chat_photo) => {
                        if let Err(err) = sender.send(TgUpdate::ChatPhoto(chat_photo)).await {
                            warn!("{}", err)
                        };
                    }
                    TdType::UpdateChatTitle(chat_title) => {
                        if let Err(err) = sender.send(TgUpdate::ChatTitle(chat_title)).await {
                            warn!("{}", err)
                        };
                    }
                    TdType::UpdateFile(file) => {
                        if file.file().local().is_downloading_completed() {
                            trace!("file {} downloading finished", file.file().id());
                            if !download_queue
                                .lock()
                                .unwrap()
                                .is_in_progress(&file.file().id())
                            {
                                continue;
                            }
                            let file_id = match download_queue
                                .lock()
                                .unwrap()
                                .mark_as_done_and_get_new(&file.file().id())
                            {
                                None => continue,
                                Some(file_id) => file_id,
                            };
                            trace!("file {} downloading started", file_id);
                            if let Err(e) =
                                api.download_file(make_download_file_request(file_id)).await
                            {
                                error!("{}", e);
                            }
                            if let Err(err) = sender.send(TgUpdate::FileDownloaded(file)).await {
                                error!("{}", err);
                            };
                        } else {
                            let percent = file.file().local().downloaded_size() / (file.file().expected_size() / 100);
                            debug!("file {} downloaded percent: {}", file.file().id(), percent);
                        }
                    }
                    _ => {}
                }
            }
        });
        Ok(())
    }

    pub async fn start(&mut self) -> Result<JoinHandle<ClientState>> {
        self.client.start().await
    }

    pub async fn get_chat(&self, chat_id: &i64) -> Result<Chat> {
        Ok(self
            .api
            .get_chat(GetChat::builder().chat_id(*chat_id).build())
            .await?)
    }

    pub async fn search_public_chats(&self, query: &str) -> Result<Vec<types::Channel>> {
        let chats = self
            .api
            .search_public_chats(SearchPublicChats::builder().query(query).build())
            .await?;
        Ok(self.convert_chats_to_channels(chats).await?)
    }

    pub async fn join_chat(&self, chat_id: &i64) -> Result<Ok> {
        Ok(self
            .api
            .join_chat(JoinChat::builder().chat_id(*chat_id).build())
            .await?)
    }

    pub async fn download_file(&mut self, file_id: i64) -> Result<()> {
        let may_be_download = {
            let mut queue = self.download_queue.lock().unwrap();
            queue.may_be_download(file_id)
        };
        if may_be_download {
            self.api
                .download_file(make_download_file_request(file_id))
                .await?;
        }
        Ok(())
    }

    pub async fn get_channel(&self, chat_id: i64) -> Result<Option<types::Channel>> {
        let chat = self
            .api
            .get_chat(GetChat::builder().chat_id(chat_id).build())
            .await?;
        match &chat.type_() {
            ChatType::Supergroup(sg) if sg.is_channel() => {
                let sg_info = self
                    .api
                    .get_supergroup_full_info(
                        GetSupergroupFullInfo::builder()
                            .supergroup_id(sg.supergroup_id())
                            .build(),
                    )
                    .await?;
                let sg = self
                    .api
                    .get_supergroup(
                        GetSupergroup::builder()
                            .supergroup_id(sg.supergroup_id())
                            .build(),
                    )
                    .await?;
                Ok(Some(types::Channel::convert(&chat, &sg, &sg_info)))
            }
            _ => Ok(None),
        }
    }

    async fn convert_chats_to_channels(&self, chats: Chats) -> Result<Vec<types::Channel>> {
        let channels = join_all(chats.chat_ids().iter().map(|&c| self.get_channel(c))).await;
        let mut result = vec![];
        for channel in channels {
            if let Some(ch) = channel? {
                result.push(ch)
            }
        }
        Ok(result)
    }

    pub async fn get_all_channels(&self, limit: i64) -> Result<Vec<types::Channel>> {
        let chats = self
            .api
            .get_chats(
                GetChats::builder()
                    .limit(limit)
                    .offset_order(9223372036854775807)
                    .build(),
            )
            .await?;
        Ok(self.convert_chats_to_channels(chats).await?)
    }

    pub async fn get_chat_history(
        &self,
        chat_id: i64,
        offset: i64,
        limit: i64,
        message_id: i64,
    ) -> Result<Messages> {
        Ok(self
            .api
            .get_chat_history(
                GetChatHistory::builder()
                    .chat_id(chat_id)
                    .offset(offset)
                    .limit(limit)
                    .from_message_id(message_id)
                    .build(),
            )
            .await?)
    }

    pub async fn get_message_link(&self, chat_id: i64, message_id: i64) -> Result<String> {
        Ok(self
            .api
            .get_message_link(
                GetMessageLink::builder()
                    .chat_id(chat_id)
                    .message_id(message_id)
                    .build(),
            )
            .await?
            .url()
            .clone())
    }

    pub fn get_chat_history_stream(
        client: Arc<RwLock<TgClient>>,
        chat_id: i64,
        date: i64,
    ) -> impl Stream<Item = Result<Message>> {
        futures::stream::unfold(
            (i64::MAX, client),
            move |(mut from_message_id, client)| async move {
                let guard = client.clone();
                let api = guard.read().await;
                let history = api.get_chat_history(chat_id, 0, 10, from_message_id).await;
                let result_messages: Result<Vec<Message>>;
                match history {
                    Ok(messages) => {
                        result_messages = Ok(messages
                            .messages()
                            .iter()
                            .filter_map(|msg| match msg {
                                None => None,
                                Some(msg) => {
                                    if msg.date() < date {
                                        None
                                    } else {
                                        if msg.id() < from_message_id {
                                            from_message_id = msg.id()
                                        }
                                        Some(msg.clone())
                                    }
                                }
                            })
                            .collect());
                    }
                    Err(err) => result_messages = Err(err),
                };
                match result_messages {
                    Ok(messages) => {
                        if !messages.is_empty() {
                            Some((Ok(messages), (from_message_id, client)))
                        } else {
                            None
                        }
                    }
                    Err(err) => Some((Err(err), (from_message_id, client))),
                }
            },
        )
        .map_ok(|updates| futures::stream::iter(updates).map(Ok))
        .try_flatten()
    }

    pub async fn close(&mut self) {
        self.api.close(Close::builder().build()).await.unwrap();
    }
}

fn make_download_file_request(file_id: i64) -> DownloadFile {
    DownloadFile::builder()
        .file_id(file_id)
        .synchronous(false)
        .priority(1)
        .build()
}

#[derive(Debug)]
pub enum TgUpdate {
    NewMessage(UpdateNewMessage),
    MessageContent(UpdateMessageContent),
    ChatPhoto(UpdateChatPhoto),
    FileDownloaded(UpdateFile),
    ChatTitle(UpdateChatTitle),
    // looks like we do not need it: that updates may contain data, which does not make sense for project
    // there is just a several improtant fields: description, username and invite_link
    // Supergroup(UpdateSupergroup),
    // SupergroupFullInfo(UpdateSupergroupFullInfo),
}

#[cfg(test)]
mod tests {
    use crate::tg_client::{get_update_file_handler, TgClient, Config};
    use crate::traits;
    use async_trait::async_trait;
    use rust_tdlib::client::api::Api;
    use rust_tdlib::errors::{RTDError, RTDResult};
    use rust_tdlib::types::*;
    use std::sync::{Arc, Condvar, Mutex};
    use tokio::sync::{mpsc, Mutex as AsyncMutex, RwLock};

    #[derive(Clone)]
    struct MockedApi;

    #[async_trait]
    impl traits::TelegramAsyncApi for MockedApi {
        async fn download_file(&self, download_file: DownloadFile) -> RTDResult<File> {
            Ok(File::builder().build())
        }

        async fn close(&self, close: Close) -> RTDResult<Ok> {
            Ok(Ok::builder().build())
        }

        async fn get_chat(&self, get_chat: GetChat) -> RTDResult<Chat> {
            Ok(Chat::builder().build())
        }

        async fn get_chats(&self, get_chats: GetChats) -> RTDResult<Chats> {
            Ok(Chats::builder().build())
        }

        async fn get_chat_history(&self, get_chat_history: GetChatHistory) -> RTDResult<Messages> {
            Ok(Messages::builder().build())
        }

        async fn get_message_link(&self, get_message_link: GetMessageLink) -> RTDResult<HttpUrl> {
            Ok(HttpUrl::builder().build())
        }

        async fn search_public_chats(
            &self,
            search_public_chats: SearchPublicChats,
        ) -> RTDResult<Chats> {
            Ok(Chats::builder().build())
        }

        async fn join_chat(&self, join_chat: JoinChat) -> RTDResult<Ok> {
            Ok(Ok::builder().build())
        }

        async fn get_supergroup_full_info(
            &self,
            get_supergroup_full_info: GetSupergroupFullInfo,
        ) -> RTDResult<SupergroupFullInfo> {
            Ok(SupergroupFullInfo::builder().build())
        }

        async fn get_supergroup(&self, get_supergroup: GetSupergroup) -> RTDResult<Supergroup> {
            Ok(Supergroup::builder().build())
        }
    }

    #[tokio::test]
    async fn test_download_file() {
        let mut client = TgClient::new(&Config {
            max_download_queue_size: 1,
            log_download_state_secs_interval: 100,
            log_verbosity_level: 0,
            encryption_key: "",
            database_directory: "",
            api_id: 0,
            api_hash: "",
            phone_number: "",
        });
        client.api = Box::new(MockedApi);

        client.download_file(1).await;
        assert_eq!(client.download_queue.lock().unwrap().queue().len(), 0);
        assert_eq!(
            client.download_queue.lock().unwrap().in_progress(),
            &vec![1]
        );

        client.download_file(1).await;
        assert_eq!(client.download_queue.lock().unwrap().queue(), &vec![1]);
        assert_eq!(
            client.download_queue.lock().unwrap().in_progress(),
            &vec![1]
        );

        client.download_file(2).await;
        assert_eq!(client.download_queue.lock().unwrap().queue(), &vec![1, 2]);
        assert_eq!(
            client.download_queue.lock().unwrap().in_progress(),
            &vec![1]
        );

        let (sender, receiver) = mpsc::channel(1);
        let chann = Arc::new(AsyncMutex::new(sender));
        let download_finished_handler =
            get_update_file_handler(chann, client.download_queue.clone());
        let eapi = EventApi::new(Api::default());

        download_finished_handler((
            &eapi,
            &UpdateFile::builder()
                .file(
                    File::builder()
                        .local(LocalFile::builder().is_downloading_completed(false).build()),
                )
                .build(),
        ));

        // no changes
        assert_eq!(client.download_queue.lock().unwrap().queue(), &vec![1, 2]);
        assert_eq!(
            client.download_queue.lock().unwrap().in_progress(),
            &vec![1]
        );

        download_finished_handler((
            &eapi,
            &UpdateFile::builder()
                .file(
                    File::builder()
                        .id(10)
                        .local(LocalFile::builder().is_downloading_completed(true).build()),
                )
                .build(),
        ));

        // no changes: file not in progress
        assert_eq!(client.download_queue.lock().unwrap().queue(), &vec![1, 2]);
        assert_eq!(
            client.download_queue.lock().unwrap().in_progress(),
            &vec![1]
        );

        download_finished_handler((
            &eapi,
            &UpdateFile::builder()
                .file(
                    File::builder()
                        .id(1)
                        .local(LocalFile::builder().is_downloading_completed(true).build()),
                )
                .build(),
        ));
        assert_eq!(client.download_queue.lock().unwrap().queue(), &vec![2]);
        assert_eq!(
            client.download_queue.lock().unwrap().in_progress(),
            &vec![1]
        );

        download_finished_handler((
            &eapi,
            &UpdateFile::builder()
                .file(
                    File::builder()
                        .id(1)
                        .local(LocalFile::builder().is_downloading_completed(true).build()),
                )
                .build(),
        ));
        assert_eq!(client.download_queue.lock().unwrap().queue().len(), 0);
        assert_eq!(
            client.download_queue.lock().unwrap().in_progress(),
            &vec![2]
        );

        client.download_file(3).await;
        client.download_file(4).await;
        client.download_file(5).await;

        download_finished_handler((
            &eapi,
            &UpdateFile::builder()
                .file(
                    File::builder()
                        .id(2)
                        .local(LocalFile::builder().is_downloading_completed(true).build()),
                )
                .build(),
        ));

        assert_eq!(client.download_queue.lock().unwrap().queue(), &vec![4, 5]);
        assert_eq!(
            client.download_queue.lock().unwrap().in_progress(),
            &vec![3]
        );
    }
}
