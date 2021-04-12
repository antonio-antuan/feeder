use std::sync::Arc;

use chrono::{NaiveDate, NaiveDateTime};
use futures::stream::StreamExt;
use tg_collector::tg_client::{TgClient, TgUpdate};
use tokio::sync::{mpsc, Mutex, RwLock};
use tokio::task::spawn;

#[tokio::main]
async fn main() {
    env_logger::init();
    let (sender, receiver) = mpsc::channel::<TgUpdate>(2000);
    let mut client = TgClient::builder()
        .with_api_hash(env!("API_HASH").to_string())
        .with_api_id(env!("API_ID").parse::<i64>().unwrap())
        .with_phone_number(env!("TG_PHONE").to_string())
        .with_encryption_key(env!("ENCRYPTION_KEY").to_string())
        .build()
        .unwrap();
    client.start_listen_updates(sender).await.unwrap();
    client.start().await.unwrap();
    let chats = client.search_public_chats("profunctor").await.unwrap();
    println!("chats: {:?}", chats);

    let mut all_chats = client.get_all_channels(10).await.unwrap();
    println!("{:?}", all_chats);

    let receiver = Mutex::new(receiver);
    spawn(async move {
        loop {
            let update = receiver.lock().await.recv().await;
            println!("{:?}", update);
        }
    });

    let mut history_cursor = Box::pin(TgClient::get_chat_history_stream(
        Arc::new(RwLock::new(client)),
        all_chats.pop().unwrap().chat_id,
        NaiveDate::from_ymd(2021, 2, 17)
            .and_hms(0, 0, 0)
            .timestamp() as i32,
    ));
    while let Some(message) = history_cursor.next().await {
        println!(
            "{:?}",
            NaiveDateTime::from_timestamp(message.unwrap().date() as i64, 0)
        );
    }
}
