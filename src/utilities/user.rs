use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};

use crate::error::Error;

/// User Site Feed
/// TODO: Option<String> => Option<Url>
#[derive(Deserialize, Serialize)]
pub struct Feed {
    json: Option<String>,
    atom: Option<String>,
    rss: Option<String>,
}

/// 从网站获取 Feed 链接
pub async fn get_site_feed(url: String) -> Result<Feed, Error> {
    let response = reqwest::get(url).await?;
    let text = response.text().await?;
    let document = Html::parse_document(&text);

    /// TODO: 如果是相对链接，则转为绝对链接
    fn feed_auto_discovery(document: &Html, kind: &str) -> Option<String> {
        let selector = Selector::parse(&format!("link[rel=\"alternate\"][type=\"{}\"]", kind)).unwrap();
        let link = document.select(&selector)
            .next()
            .unwrap()
            .value()
            .attr("href")
            .unwrap()
            .to_string();

        Some(link)
    }

    let feed = Feed {
        json: feed_auto_discovery(&document, "application/feed+json"),
        atom: feed_auto_discovery(&document, "application/atom+xml"),
        rss: feed_auto_discovery(&document, "application/rss+xml"),
    };

  Ok(feed)
}