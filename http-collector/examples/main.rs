fn main() {
    use scraper::{Html, Selector};

    let doc = Html::parse_fragment(
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
    let sel = Selector::parse(r#"a[href*="rss"]"#).unwrap();
    println!("{:?}", doc.select(&sel).next());
}
