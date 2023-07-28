use scraper::{Html, Selector, ElementRef};
use serde::{Deserialize, Serialize};

use crate::{
    error::AppError,
    utilities::absolutize_relative_url,
};

/// User Site Feed
#[derive(Deserialize, Serialize)]
pub struct Feed {
    pub json: Option<String>,
    pub atom: Option<String>,
    pub rss: Option<String>,
}

/// 从网站获取 Feed 链接
pub async fn get_site_feed(domain: String) -> Result<Feed, AppError> {
    let response = reqwest::get(format!("https://{}", &domain)).await?;
    let text = response.text().await?;
    let document = Html::parse_document(&text);
    let head = document.select(&Selector::parse("head").unwrap()).next().unwrap();

    fn feed_auto_discovery(head: &ElementRef, domain: &str, kind: &str) -> Result<Option<String>, AppError> {
        let selector = Selector::parse(&format!("link[rel=\"alternate\"][type=\"{}\"]", kind)).unwrap();
        let link_href = head.select(&selector)
            .next()
            .and_then(|link| link.value().attr("href"))
            .and_then(|href| Some(absolutize_relative_url(href.to_string(), domain.to_string()).unwrap().to_string()));

        Ok(link_href)
    }

    let feed = Feed {
        json: feed_auto_discovery(&head, &domain, "application/feed+json")?,
        atom: feed_auto_discovery(&head, &domain,"application/atom+xml")?,
        rss: feed_auto_discovery(&head, &domain, "application/rss+xml")?,
    };

    Ok(feed)
}