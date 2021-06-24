use reqwest::Client;
use vk_auth::Authorizer;
use vk_collector::client::VkClient;

#[tokio::main]
async fn main() {
    let client = Client::builder().cookie_store(true).build().unwrap();
    let auth = Authorizer::builder()
        .with_client(client.clone())
        .build()
        .unwrap();
    let token = auth
        .get_token(env!("APP_ID"), env!("EMAIL"), env!("PASSWORD"))
        .await
        .unwrap();
    let cl = VkClient::new(token.access_token(), client, 3, 1);
    let res = cl.get_wall(-1, 1, 1).await.unwrap();
    println!("{:?}", res);
    let res = cl.search_group("rust lang", 0, 10).await.unwrap();
    println!("{:?}", res);
}
