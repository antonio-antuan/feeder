use reqwest;
use async_trait::async_trait;
use crate::result;
use std::future::Future;
use crate::types::{VkResponse, WallItem, Group};
use serde::de::DeserializeOwned;
use reqwest::Error;

macro_rules! res_or_err {
    ($res:ident) => {
        Ok(match $res {
                VkResponse::Error(e) => {return Err(result::Error::VkError(e))}
                VkResponse::Response(r) => {r.items}
            })
    }
}

#[async_trait]
pub trait Response {
    async fn json<T: DeserializeOwned>(self) -> reqwest::Result<T>;
    async fn text(self) -> reqwest::Result<String>;
}

#[async_trait]
impl Response for reqwest::Response {
    async fn json<T: DeserializeOwned>(self) -> reqwest::Result<T> {
        self.json().await
    }

    async fn text(self) -> reqwest::Result<String> {
        self.text().await
    }
}

#[async_trait]
pub trait RequestBuilder<R>
    where R: Response
{
    async fn send(self) -> Result<R, reqwest::Error>;
}

#[async_trait]
impl RequestBuilder<reqwest::Response> for reqwest::RequestBuilder {
    async fn send(self) -> Result<reqwest::Response, Error> {
        self.send().await
    }
}

pub trait HttpClient<B, R>
    where
        B: RequestBuilder<R>,
        R: Response
{
    fn get<U: reqwest::IntoUrl>(&self, url: U) -> B;
}

impl HttpClient<reqwest::RequestBuilder, reqwest::Response> for reqwest::Client {
    fn get<U: reqwest::IntoUrl>(&self, url: U) -> reqwest::RequestBuilder {
        self.get(url)
    }
}

pub struct VkClient<C>
{
    client: C,
    token: String,
}

impl<C> VkClient<C> {
    pub fn new(token: &str, client: C) -> Self {
        Self { client, token: token.to_string() }
    }

    pub async fn get_wall<B, R>(&self, owner_id: i64, offset: u8, count: u8) -> result::Result<Vec<WallItem>>
    where
        C: HttpClient<B, R>,
        B: RequestBuilder<R>,
        R: Response

    {
        let url = format!(
            "https://api.vk.com/method/wall.get/?\
            owner_id={owner}&\
            access_token={token}&\
            offset={offset}&\
            count={count}&\
            v={v}",
                          owner = owner_id, token = self.token, v = "5.21", count=count, offset=offset
        );
        let r: VkResponse<WallItem> = self.client.get(&url).send().await?.json().await?;
        res_or_err!(r)
    }


    pub async fn search_group<B, R>(&self, q: &str, offset: u8, count: u8) -> result::Result<Vec<Group>>
    where
        C: HttpClient<B, R>,
        B: RequestBuilder<R>,
        R: Response

    {
        let url = format!(
            "https://api.vk.com/method/groups.search/?\
            q={q}&\
            access_token={token}&\
            offset={offset}&\
            count={count}&\
            v={v}",
                          q = q, token = self.token, v = "5.21", count=count, offset=offset
        );
        let r: VkResponse<Group> = self.client.get(&url).send().await?.json().await?;
        res_or_err!(r)
    }
}