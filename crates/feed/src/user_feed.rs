use hatsu_db_schema::user::Model as DbUser;
use hatsu_utils::AppError;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::UserFeedItem;

/// JSON Feed 1.1
///
/// <https://www.jsonfeed.org/version/1.1/#top-level-a-name-top-level-a>
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct UserFeed {
    #[serde(rename = "_hatsu")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hatsu: Option<UserFeedHatsu>,
    pub feed_url: Url,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_url: Option<Url>,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    pub items: Vec<UserFeedItem>,
}

/// Hatsu JSON Feed Extension
///
/// <https://github.com/importantimport/hatsu/issues/1>
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct UserFeedHatsu {
    pub about: Option<Url>,
    pub banner_image: Option<Url>,
}

impl UserFeed {
    pub async fn get(user: DbUser) -> Result<Self, AppError> {
        match user {
            DbUser {
                feed_json: Some(url),
                ..
            } => Ok(Self::parse_json_feed(Url::parse(&url)?).await?),
            DbUser {
                feed_atom: Some(url),
                ..
            } => Ok(Self::parse_xml_feed(Url::parse(&url)?).await?),
            DbUser {
                feed_rss: Some(url),
                ..
            } => Ok(Self::parse_xml_feed(Url::parse(&url)?).await?),
            DbUser {
                feed_json: None,
                feed_atom: None,
                feed_rss: None,
                ..
            } => Err(AppError::not_found("Feed Url", &user.name)),
        }
    }

    #[async_recursion::async_recursion]
    pub async fn get_full(self) -> Result<Self, AppError> {
        match self.next_url {
            Some(url) => {
                let next_feed = Self::parse_json_feed(url).await?;
                let feed = Self {
                    next_url: next_feed.next_url,
                    items: [self.items, next_feed.items].concat(),
                    ..self
                };

                Ok(Self::get_full(feed).await?)
            },
            None => Ok(self),
        }
    }

    pub async fn parse_json_feed(feed_url: Url) -> Result<Self, AppError> {
        Ok(reqwest::get(feed_url).await?.json::<Self>().await?)
    }

    pub async fn parse_xml_feed(feed_url: Url) -> Result<Self, AppError> {
        let feed = feed_rs::parser::parse(
            reqwest::get(feed_url.clone())
                .await?
                .text()
                .await?
                .as_bytes(),
        )?;

        let items = feed
            .entries
            .iter()
            .map(|entry| UserFeedItem::from_entry(entry))
            .collect();

        Ok(Self {
            feed_url,
            next_url: None,
            hatsu: None,
            title: match feed.title {
                Some(title) => title.content,
                None => String::from("untitled"),
            },
            description: feed.description.map(|text| text.content),
            icon: feed.icon.map(|image| Url::parse(&image.uri).unwrap()),
            language: feed.language,
            items,
        })
    }
}
