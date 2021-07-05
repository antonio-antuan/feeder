use reqwest::Client;
use sea_query::Iden;
use vk_collector::client::VkClient;

#[derive(Debug, Iden)]
pub struct Foo {
    id: i32,
    name: String,
    value: Option<i32>,
}

#[tokio::main]
async fn main() {
    let token = env!("VK_TOKEN");
    let cl = VkClient::new(token, Client::new(), 3, 1);
    let res = cl.get_wall(-1, 1, 1).await.unwrap();
    println!("{:?}", res);
    let res = cl.search_group("rust lang", 0, 10).await.unwrap();
    println!("{:?}", res);
}
