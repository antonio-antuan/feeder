use reqwest::Client;
use vk_collector::collector::VkClient;


#[tokio::main]
async fn main() {
    let token = env!("VK_TOKEN");
    let cl = VkClient::new(token, Client::new());
    let res = cl.get_wall(-1, 1, 1).await.unwrap();
    println!("{:?}", res);
   let res = cl.search_group("rust lang", 0, 10).await.unwrap();
    println!("{:?}", res);
}