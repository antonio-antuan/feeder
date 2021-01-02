use async_trait::async_trait;

use atom_syndication::Feed as AtomFeed;
use futures::future::join_all;
use reqwest::{Client, Response};
use rss::Channel;
use scraper::{Html, Selector};
use std::str::FromStr;
use url::Url;

use crate::models::*;
use crate::result::{Error, Result};
use tokio::sync::mpsc;

#[async_trait]
pub trait ResultsHandler {
    async fn process(&self, result: Result<(&Feed, FeedKind, String)>);
}

#[async_trait]
pub trait Cache {
    async fn get(&self, link: &str) -> Option<FeedKind>;
    async fn set(&self, link: &str, feed_kind: &FeedKind) -> Result<()>;
}

pub struct CacheStub {}

#[async_trait]
impl Cache for CacheStub {
    async fn get(&self, _link: &str) -> Option<FeedKind> {
        None
    }

    async fn set(&self, _link: &str, _feed_kind: &FeedKind) -> Result<()> {
        Ok(())
    }
}

#[derive(Clone)]
pub struct HttpCollector<C: Cache> {
    client: Client,
    cache: C,
}

impl Default for HttpCollector<CacheStub> {
    fn default() -> Self {
        Self::new()
    }
}

impl HttpCollector<CacheStub> {
    pub fn new() -> HttpCollector<CacheStub> {
        HttpCollector {
            client: Client::new(),
            cache: CacheStub {},
        }
    }
}

impl<C> HttpCollector<C>
where
    C: Cache,
{
    pub fn with_cache(&mut self, cache: C) {
        self.cache = cache
    }

    pub async fn run(
        &self,
        mut sources_receiver: mpsc::Receiver<Vec<(Option<FeedKind>, String)>>,
        process_results: &impl ResultsHandler,
    ) {
        while let Some(sources) = sources_receiver.recv().await {
            debug!("retrieve sources: {}", sources.len());
            let mut tasks = vec![];
            for (kind, link) in sources {
                debug!("want to scrape: ({:?}) {}", kind, link);
                tasks.push(self.scrape_and_process_content(kind, link, process_results));
            }
            join_all(tasks).await;
        }
    }

    async fn scrape_and_process_content(
        &self,
        kind: Option<FeedKind>,
        link: String,
        process_results: &impl ResultsHandler,
    ) {
        match self.scrape_feed(kind, link.as_str()).await {
            Ok(content) => {
                process_results
                    .process(Ok((&content, content.kind, link)))
                    .await
            }
            Err(err) => process_results.process(Err(err)).await,
        };
    }

    async fn scrape_unknown_feed_kind(&self, link: &str) -> Result<Feed> {
        let content = self.scrape(link).await?;
        let feeds = self.traverse_parsers(link, content.as_str());
        match feeds.len() {
            0 => Err(Error::NoFeed),
            1 => Ok(feeds.first().unwrap().clone()),
            _ => Ok(feeds
                .iter()
                .find(|f| f.kind == FeedKind::RSS)
                .or_else(|| Some(feeds.first().unwrap()))
                .unwrap()
                .clone()),
        }
    }

    async fn scrape_feed(&self, kind: Option<FeedKind>, link: &str) -> Result<Feed> {
        let kind = match kind {
            None => match self.cache.get(link).await {
                None => {
                    let feed = self.scrape_unknown_feed_kind(link).await?;
                    if let Err(e) = self.cache.set(link, &feed.kind).await {
                        warn!("cache not set: {:?}", e);
                    };
                    return Ok(feed);
                }
                Some(kind) => kind,
            },
            Some(kind) => kind,
        };
        let result = match kind {
            FeedKind::RSS => self.scrape_rss(link).await?,
            FeedKind::Atom => self.scrape_atom(link).await?,
            FeedKind::WP => return Err(Error::SourceNotSupported),
        };
        Ok(result)
    }

    async fn scrape_rss(&self, link: &str) -> Result<Feed> {
        parse_rss_feed(link, self.scrape(link).await?.as_str())
    }

    async fn scrape_atom(&self, link: &str) -> Result<Feed> {
        parse_atom_feed(link, self.scrape(link).await?.as_str())
    }

    async fn scrape(&self, link: &str) -> Result<String> {
        debug!("start scrape {} {:?}", link, std::thread::current().id());
        let response: Response = self.client.get(link).send().await?;
        debug!("scraped {} {:?}", link, std::thread::current().id());
        Ok(response.text().await?)
    }

    fn detect_possible_feeds(
        &self,
        link: &str,
        parsed_doc: Html,
    ) -> Result<Vec<(String, FeedKind)>> {
        let page_scrape_url = Url::parse(link)?;
        let mut for_check = CheckLinks::new(&page_scrape_url);

        // get link from specified links in <head></head>
        for (kind, selector) in vec![
            (
                FeedKind::RSS,
                Selector::parse(r#"link[type="application/rss+xml"]"#).unwrap(),
            ),
            (
                FeedKind::Atom,
                Selector::parse(r#"link[type="application/atom+xml"]"#).unwrap(),
            ),
        ] {
            for element in parsed_doc.select(&selector) {
                if let Some(href) = element.value().attr("href") {
                    for_check.add_check_with_href(href, kind)
                };
            }
        }

        // get wordpress links from <head></head>
        let wp_selector = Selector::parse(r#"link[rel="https://api.w.org/"]"#).unwrap();
        if let Some(found_wp) = parsed_doc.select(&wp_selector).next() {
            if let Some(href) = found_wp.value().attr("href") {
                let parsed_href = Url::parse(href)?;

                for_check.push((parsed_href.join("wp/v2/posts")?.to_string(), FeedKind::WP));
                for_check.push((
                    page_scrape_url.join("wp/v2/posts")?.to_string(),
                    FeedKind::WP,
                ));

                // if rss feeds not found yet, then we're going to check WP-provided RSS-feeds
                let f = "feed/";
                if !for_check.has_link_ends_with(f) {
                    for_check.push((parsed_href.join(f)?.to_string(), FeedKind::RSS));
                    for_check.push((page_scrape_url.join(f)?.to_string(), FeedKind::RSS));
                };
            };
        };
        {
            if for_check.is_empty() {
                let selector = Selector::parse(r#"a[href*="rss"]"#).unwrap();
                let x = parsed_doc.select(&selector);
                for element in x {
                    if let Some(href) = element.value().attr("href") {
                        for_check.add_check_with_href(href, FeedKind::RSS)
                    };
                }
            };
        }
        Ok(for_check.consume_results())
    }

    pub fn traverse_parsers(&self, link: &str, content: &str) -> Vec<Feed> {
        let mut result = vec![];
        let parsers: Vec<&dyn Fn(&str, &str) -> Result<Feed>> =
            vec![&parse_rss_feed, &parse_atom_feed];
        for parser in parsers {
            match parser(link, content) {
                Ok(feed) => {
                    result.push(feed);
                }
                Err(err) => {
                    trace!("not parsed: {}", err);
                }
            }
        }
        result
    }

    pub async fn detect_feeds(&self, link: &str) -> Result<Vec<Feed>> {
        let content = self.scrape(link).await?;

        let mut result = self.traverse_parsers(link, content.as_str());
        let favicon;
        let for_check = {
            let parsed_doc = Html::parse_document(content.as_str());
            let icon_selector = Selector::parse("link[rel=\"icon\"]").unwrap();
            favicon = match parsed_doc.select(&icon_selector).next() {
                None => None,
                Some(node) => node.value().attr("href").map(|h| h.to_string()),
            };
            self.detect_possible_feeds(link, parsed_doc)?
        };

        let mut checks = vec![];
        for_check
            .iter()
            .map(|feed| {
                debug!("going to check {:?}", feed);
                checks.push(self.scrape_feed(Some(feed.1), feed.0.as_str()));
            })
            .for_each(drop);
        join_all(checks)
            .await
            .into_iter()
            .map(|check_result| match check_result {
                Ok(mut feed) => {
                    match feed.image {
                        None => feed.image = favicon.clone(),
                        Some(_) => {}
                    };
                    result.push(feed);
                }
                Err(err) => {
                    error!("{:?}", err);
                }
            })
            .for_each(drop);
        Ok(result)
    }
}

struct CheckLinks<'a> {
    for_check: Vec<(String, FeedKind)>,
    page_scrape_url: &'a Url,
}

impl<'a> CheckLinks<'a> {
    pub fn consume_results(self) -> Vec<(String, FeedKind)> {
        self.for_check
    }

    pub fn has_link_ends_with(&self, ends: &str) -> bool {
        self.for_check.iter().any(|(link, _)| link.ends_with(ends))
    }

    pub fn is_empty(&self) -> bool {
        self.for_check.is_empty()
    }

    pub fn new(page_scrape_url: &'a Url) -> Self {
        Self {
            page_scrape_url,
            for_check: vec![],
        }
    }
    pub fn push(&mut self, pair: (String, FeedKind)) {
        self.for_check.push(pair)
    }

    pub fn add_check_with_href(&mut self, href: &str, kind: FeedKind) {
        let link = if href.starts_with('/') {
            self.page_scrape_url.join(href).unwrap().into_string()
        } else {
            href.to_string()
        };
        self.for_check.push((link, kind));
    }
}
fn get_image(content: &str) -> Option<String> {
    let image_selector = Selector::parse("img").unwrap();
    let parsed_doc = Html::parse_document(content);
    let image = match parsed_doc.select(&image_selector).next() {
        None => None,
        Some(img) => img.value().attr("src").map(|x| x.to_string()),
    };
    image
}

fn current_time() -> chrono::DateTime<chrono::FixedOffset> {
    let local_time = chrono::Local::now();
    let utc_time = chrono::DateTime::<chrono::Utc>::from_utc(local_time.naive_utc(), chrono::Utc);
    utc_time.with_timezone(&chrono::FixedOffset::east(0))
}

fn get_feed_pub_date(pub_date: Option<&str>) -> chrono::NaiveDateTime {
    let pub_date = chrono::DateTime::parse_from_rfc2822(pub_date.unwrap_or_default())
        .unwrap_or_else(|_| current_time());
    pub_date.naive_utc()
}

fn parse_atom_feed(link: &str, content: &str) -> Result<Feed> {
    let channel = AtomFeed::from_str(content)?;
    let image = match channel.icon() {
        None => None,
        Some(img) => Some(img.to_string()),
    };
    let mut feed_items = vec![];
    for item in channel.entries() {
        let description = item
            .content()
            .map_or(item.summary(), |f| f.value())
            .unwrap_or_default();
        let image = get_image(description);
        feed_items.push(FeedItem {
            title: Some(item.title().to_string()),
            image_link: image,
            pub_date: item
                .published()
                .unwrap_or_else(|| item.updated())
                .naive_utc(),
            content: description.to_string(),
            guid: item.id.to_string(),
        })
    }
    Ok(Feed {
        image,
        link: link.to_string(),
        kind: FeedKind::RSS,
        name: channel.title,
        content: feed_items,
    })
}

fn parse_rss_feed(link: &str, content: &str) -> Result<Feed> {
    let channel = Channel::from_str(content)?;
    let mut feed_items: Vec<FeedItem> = vec![];
    for item in channel.items() {
        let description = item
            .content()
            .unwrap_or_else(|| item.description().unwrap_or_default());
        let mut guid = String::new();
        if item.guid().is_some() {
            guid.push_str(item.guid().unwrap().value())
        } else if item.link().is_some() {
            guid.push_str(item.link().unwrap())
        } else {
            warn!("can't get unique id for record {:?}", item);
            continue;
        }
        feed_items.push(FeedItem {
            title: item.title().map(|f| f.to_string()),
            pub_date: get_feed_pub_date(item.pub_date()),
            content: description.to_string(),
            guid,
            image_link: get_image(description),
        })
    }
    Ok(Feed {
        image: channel.image().map(|i| i.url().to_string()),
        link: link.to_string(),
        kind: FeedKind::RSS,
        name: channel.title().to_string(),
        content: feed_items,
    })
}

#[cfg(test)]
mod tests {
    use crate::collector::{get_image, HttpCollector};
    use crate::models::FeedKind;
    use scraper::Html;

    #[test]
    fn test_get_image() {
        let content = "\
        <p><img src=\"https://habrastorage.org/webt/4n/c1/v0/4nc1v0ifaa8rzyrzq5q1q7r4t8q.png\"></p>
        <br>
        <p>В этой статье я хочу разобрать один из самых популярных опенсорс-инструментов,
        <a href=\"https://nodered.org\" rel=\"nofollow\">Node-RED</a>,\
        с точки зрения создания простых прототипов приложений с минимумом программирования</p>";
        let image = get_image(content);
        assert_eq!(
            image,
            Some(
                "https://habrastorage.org/webt/4n/c1/v0/4nc1v0ifaa8rzyrzq5q1q7r4t8q.png"
                    .to_string()
            )
        )
    }

    #[test]
    fn test_detect_possible_feeds() {
        let collector = HttpCollector::new();

        let content = Html::parse_fragment(
            r#"
        <html>
        <head>
        <link type="application/rss+xml" href="https://test_detect_possible_feeds.rss">
        <link type="application/atom+xml" href="https://test_detect_possible_feeds.atom">
        <link rel="https://api.w.org/" href="https://wp-url.wp">
        </head>
        <body>
        <div>
        <a href="https://a.com/feeds/rss/"></a>
        <a href="https://a.com/feeds/rss"></a>
        <a href="/feeds/rss"></a>
        </div>
        </body>
        </html>
        "#,
        );
        let detected = collector
            .detect_possible_feeds("https://test.test", content)
            .unwrap();

        assert_eq!(
            detected,
            vec![
                (
                    "https://test_detect_possible_feeds.rss".to_string(),
                    FeedKind::RSS
                ),
                (
                    "https://test_detect_possible_feeds.atom".to_string(),
                    FeedKind::Atom
                ),
                ("https://wp-url.wp/wp/v2/posts".to_string(), FeedKind::WP),
                ("https://test.test/wp/v2/posts".to_string(), FeedKind::WP),
                ("https://wp-url.wp/feed/".to_string(), FeedKind::RSS),
                ("https://test.test/feed/".to_string(), FeedKind::RSS),
            ]
        );

        let content = Html::parse_fragment(
            r#"
                <html>
                <head>
                </head>
                <body>
                <div>
                <a href="https://a.com/feeds/rss/"></a>
                <a href="https://a.com/feeds/rss"></a>
                <a href="/feeds/rss"></a>
                </div>
                </body>
                </html>
                "#,
        );

        let detected = collector
            .detect_possible_feeds("https://test.test", content)
            .unwrap();
        assert_eq!(
            detected,
            vec![
                ("https://a.com/feeds/rss/".to_string(), FeedKind::RSS),
                ("https://a.com/feeds/rss".to_string(), FeedKind::RSS),
                ("https://test.test/feeds/rss".to_string(), FeedKind::RSS)
            ]
        );
    }
}
