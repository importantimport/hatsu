use activitypub_federation::{
    config::Data,
    fetch::object_id::ObjectId
};
use hatsu_db_schema::user_feed_item::Model as DbUserFeedItem;
use hatsu_utils::{AppData, AppError};
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use url::Url;

use super::ApubUser;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ApubUserFeedItem(pub(crate) DbUserFeedItem);

impl AsRef<DbUserFeedItem> for ApubUserFeedItem {
    fn as_ref(&self) -> &DbUserFeedItem {
        &self.0
    }
}

impl Deref for ApubUserFeedItem {
    type Target = DbUserFeedItem;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<DbUserFeedItem> for ApubUserFeedItem {
    fn from (u: DbUserFeedItem) -> Self {
        Self(u)
    }
}

impl ApubUserFeedItem {
    // 转换为 JSON
    pub fn into_json(self) -> Result<JsonUserFeedItem, AppError> {
        Ok(JsonUserFeedItem {
            id: self.id.clone(),
            url: None,
            title: self.title.clone(),
            summary: self.summary.clone(),
            language: self.language.clone(),
            tags: self.tags.clone().map(|tags| serde_json::from_str(&tags).unwrap()),
            date_published: self.date_published.clone(),
            date_modified: self.date_modified.clone(),
        })
    }

    // 从 JSON 转换为本地格式
    pub fn from_json(
        json: JsonUserFeedItem,
        user_id: ObjectId<ApubUser>,
        data: &Data<AppData>
        // object_id: Option<ObjectId<DbPost>>
    ) -> Result<Self, AppError> {
        let id = json.url.unwrap_or_else(|| Url::parse(&json.id).unwrap()).to_string();

        let user_feed_item = DbUserFeedItem {
            id: id.clone(),
            user_id: user_id.inner().to_string(),
            // object_id: object_id.map(|object_id| object_id.inner().to_string()),
            object_id: Some(hatsu_utils::url::generate_object_url(data.domain(), id)?.to_string()),
            title: json.title,
            summary: json.summary,
            language: json.language,
            tags: json.tags.map(|tags| serde_json::to_string::<Vec<String>>(&tags).unwrap()),
            date_published: json.date_published,
            date_modified: json.date_modified,
        };

        Ok(user_feed_item.into())
    }
}

/// JSON Feed 1.1
/// 
/// https://www.jsonfeed.org/version/1.1/#top-level-a-name-top-level-a
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct JsonUserFeed {
    #[serde(rename = "_hatsu")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hatsu: Option<JsonUserFeedHatsu>,
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
    pub items: Vec<JsonUserFeedItem>,
}

/// Hatsu JSON Feed Extension
/// 
/// https://github.com/importantimport/hatsu/issues/1
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct JsonUserFeedHatsu {
    pub about: Option<Url>,
    pub banner_image: Option<Url>,
}

/// JSON Feed Item
/// 
/// https://www.jsonfeed.org/version/1.1/#items-a-name-items-a
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct JsonUserFeedItem {
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
