use hatsu_utils::{url::absolutize_relative_url, AppError};
use scraper::{ElementRef, Html, Selector};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::UserFeedTopLevel;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SiteFeed {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub atom: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rss: Option<Url>,
}

impl SiteFeed {
    /// # Panics
    ///
    /// No panic here.
    pub async fn get(domain: String) -> Result<Self, AppError> {
        fn feed_auto_discovery(head: &ElementRef, domain: &str, kind: &str) -> Option<Url> {
            head.select(
                &Selector::parse(&format!("link[rel=\"alternate\"][type=\"{kind}\"]")).unwrap(),
            )
            .next()
            .and_then(|link| {
                link.value()
                    .attr("href")
                    .and_then(|href| absolutize_relative_url(href, domain).ok())
            })
        }

        let response = reqwest::get(format!("https://{}", &domain)).await?;
        let text = response.text().await?;
        let document = Html::parse_document(&text);
        let head = Selector::parse("head").expect("valid selector");

        document.select(&head).next().map_or_else(
            || {
                Err(AppError::new(
                    format!("Unable to find the user's feed: {domain}"),
                    None,
                    None,
                ))
            },
            |head| {
                Ok(Self {
                    json: feed_auto_discovery(&head, &domain, "application/feed+json"),
                    atom: feed_auto_discovery(&head, &domain, "application/atom+xml"),
                    rss: feed_auto_discovery(&head, &domain, "application/rss+xml"),
                })
            },
        )
    }

    pub async fn get_user_feed(site_feed: Self, name: &str) -> Result<UserFeedTopLevel, AppError> {
        match site_feed {
            Self {
                json: Some(url), ..
            } => Ok(UserFeedTopLevel::parse_json_feed(url).await?),
            Self {
                atom: Some(url), ..
            } => Ok(UserFeedTopLevel::parse_xml_feed(url).await?),
            Self { rss: Some(url), .. } => Ok(UserFeedTopLevel::parse_xml_feed(url).await?),
            Self {
                json: None,
                atom: None,
                rss: None,
                ..
            } => Err(AppError::not_found("Feed Url", name)),
        }
    }
}
