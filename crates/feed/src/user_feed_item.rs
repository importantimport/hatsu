use std::ops::Deref;

use activitypub_federation::config::Data;
use chrono::SecondsFormat;
use feed_rs::model::Entry;
use hatsu_db_schema::{user::Model as DbUser, user_feed_item::Model as DbUserFeedItem};
use hatsu_utils::{AppData, AppError};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::UserFeedItemHatsu;

/// JSON Feed Item
///
/// <https://www.jsonfeed.org/version/1.1/#items-a-name-items-a>
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct UserFeedItem {
    #[serde(rename = "_hatsu")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hatsu: Option<UserFeedItemHatsu>,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Url>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_published: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_modified: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WrappedUserFeedItem(pub(crate) DbUserFeedItem);

impl AsRef<DbUserFeedItem> for WrappedUserFeedItem {
    fn as_ref(&self) -> &DbUserFeedItem {
        &self.0
    }
}

impl Deref for WrappedUserFeedItem {
    type Target = DbUserFeedItem;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<DbUserFeedItem> for WrappedUserFeedItem {
    fn from(u: DbUserFeedItem) -> Self {
        Self(u)
    }
}

impl WrappedUserFeedItem {
    pub fn into_json(self) -> Result<UserFeedItem, AppError> {
        Ok(UserFeedItem {
            hatsu: None,
            id: self.id.clone(),
            url: Some(Url::parse(&self.id)?),
            title: self.title.clone(),
            summary: self.summary.clone(),
            language: self.language.clone(),
            tags: self
                .tags
                .clone()
                .map(|tags| serde_json::from_str(&tags).unwrap()),
            date_published: self.date_published.clone(),
            date_modified: self.date_modified.clone(),
        })
    }

    pub fn from_json(
        json: UserFeedItem,
        user: &DbUser,
        data: &Data<AppData>,
    ) -> Result<Self, AppError> {
        let id = json
            .url
            .unwrap_or_else(|| {
                hatsu_utils::url::absolutize_relative_url(&json.id, &user.name).unwrap()
            })
            .to_string();

        let user_feed_item = DbUserFeedItem {
            hatsu: json.hatsu.map(UserFeedItemHatsu::into_db),
            id: id.clone(),
            user_id: user.id.to_string(),
            post_id: Some(hatsu_utils::url::generate_post_url(data.domain(), id)?.to_string()),
            title: json.title,
            summary: json.summary,
            language: json.language,
            tags: json
                .tags
                .map(|tags| serde_json::to_string::<Vec<String>>(&tags).unwrap()),
            date_published: json.date_published,
            date_modified: json.date_modified,
        };

        Ok(user_feed_item.into())
    }
}

impl UserFeedItem {
    #[must_use]
    pub fn from_entry(entry: &Entry) -> Self {
        Self {
            hatsu: None,
            id: entry.id.clone(),
            url: entry
                .links
                .first()
                .and_then(|link| Url::parse(&link.href).ok()),
            title: entry.title.clone().map(|text| text.content),
            summary: entry.summary.clone().map(|text| text.content),
            language: None,
            tags: entry
                .categories
                .iter()
                .map(|category| {
                    Some(
                        category
                            .label
                            .clone()
                            .unwrap_or_else(|| category.term.clone()),
                    )
                })
                .collect(),
            date_published: entry
                .published
                .map(|date| date.to_rfc3339_opts(SecondsFormat::Secs, true)),
            date_modified: entry
                .updated
                .map(|date| date.to_rfc3339_opts(SecondsFormat::Secs, true)),
        }
    }
}
