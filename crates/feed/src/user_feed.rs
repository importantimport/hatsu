use std::ops::Deref;

use hatsu_db_schema::user::UserFeed as DbUserFeed;
use hatsu_utils::{url::absolutize_relative_url, AppError};
use scraper::{ElementRef, Html, Selector};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::UserFeedTopLevel;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserFeed {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub atom: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rss: Option<Url>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WrappedUserFeed(pub(crate) DbUserFeed);

impl AsRef<DbUserFeed> for WrappedUserFeed {
    fn as_ref(&self) -> &DbUserFeed {
        &self.0
    }
}

impl Deref for WrappedUserFeed {
    type Target = DbUserFeed;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<DbUserFeed> for WrappedUserFeed {
    fn from(u: DbUserFeed) -> Self {
        Self(u)
    }
}

impl UserFeed {
    #[must_use]
    pub fn from_db(db_user_feed: DbUserFeed) -> Self {
        Self {
            json: db_user_feed.json.and_then(|url| Url::parse(&url).ok()),
            atom: db_user_feed.atom.and_then(|url| Url::parse(&url).ok()),
            rss: db_user_feed.rss.and_then(|url| Url::parse(&url).ok()),
        }
    }

    #[must_use]
    pub fn into_db(self) -> DbUserFeed {
        DbUserFeed {
            json: self.json.map(|url| url.to_string()),
            atom: self.atom.map(|url| url.to_string()),
            rss: self.rss.map(|url| url.to_string()),
        }
    }
}

impl UserFeed {
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

    pub async fn get_top_level(self, name: &str) -> Result<UserFeedTopLevel, AppError> {
        match self {
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
