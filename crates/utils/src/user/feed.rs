use scraper::{ElementRef, Html, Selector};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{url::absolutize_relative_url, AppError};

/// User Site Feed
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Feed {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub atom: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rss: Option<Url>,
}

impl Feed {
    /// 从网站获取 Feed 链接
    pub async fn get_site_feed(domain: String) -> Result<Self, AppError> {
        let response = reqwest::get(format!("https://{}", &domain)).await?;
        let text = response.text().await?;
        let document = Html::parse_document(&text);
        let head = document
            .select(&Selector::parse("head").unwrap())
            .next()
            .unwrap();

        fn feed_auto_discovery(
            head: &ElementRef,
            domain: &str,
            kind: &str,
        ) -> Result<Option<Url>, AppError> {
            let selector =
                Selector::parse(&format!("link[rel=\"alternate\"][type=\"{}\"]", kind)).unwrap();
            let link_href = head
                .select(&selector)
                .next()
                .and_then(|link| link.value().attr("href"))
                .map(|href| absolutize_relative_url(href, domain).unwrap());

            Ok(link_href)
        }

        Ok(Self {
            json: feed_auto_discovery(&head, &domain, "application/feed+json")?,
            atom: feed_auto_discovery(&head, &domain, "application/atom+xml")?,
            rss: feed_auto_discovery(&head, &domain, "application/rss+xml")?,
        })
    }
}
