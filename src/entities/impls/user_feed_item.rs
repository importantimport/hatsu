use activitypub_federation::{
    config::Data,
    fetch::object_id::ObjectId
};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{
    AppData,
    AppError,
    entities::{
        // post::Model as DbPost,
        user::Model as DbUser,
        user_feed_item::Model as DbUserFeedItem,
    },
    // utilities::absolutize_relative_url,
};

impl DbUserFeedItem {
    // 转换为 JSON
    pub async fn into_json(self) -> Result<JsonUserFeedItem, AppError> {
        Ok(JsonUserFeedItem {
            id: self.id,
            url: None,
            title: self.title,
            summary: self.summary,
            language: self.language,
            tags: self.tags.map(|tags| serde_json::from_str(&tags).unwrap()),
            date_published: self.date_published,
            date_modified: self.date_modified,
        })
    }

    // 从 JSON 转换为本地格式
    pub async fn from_json(
        json: JsonUserFeedItem,
        user_id: ObjectId<DbUser>,
        data: &Data<AppData>
        // object_id: Option<ObjectId<DbPost>>
    ) -> Result<Self, AppError> {
        let id = json.url.unwrap_or_else(|| Url::parse(&json.id).unwrap()).to_string();

        Ok(Self {
            id: id.clone(),
            user_id: user_id.inner().to_string(),
            // object_id: object_id.map(|object_id| object_id.inner().to_string()),
            object_id: Some(Url::parse(&format!("https://{}/o/{}", data.domain(), id)).unwrap().to_string()),
            title: json.title,
            summary: json.summary,
            language: json.language,
            tags: json.tags.map(|tags| serde_json::to_string::<Vec<String>>(&tags).unwrap()),
            date_published: json.date_published,
            date_modified: json.date_modified,
        })
    }
}

/// JSON Feed 1.1
/// 
/// https://www.jsonfeed.org/version/1.1/#top-level-a-name-top-level-a
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
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
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct JsonUserFeedHatsu {
    pub about: Option<Url>,
    pub banner_image: Option<Url>,
}

/// JSON Feed Item
/// 
/// https://www.jsonfeed.org/version/1.1/#items-a-name-items-a
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
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