use activitypub_federation::fetch::object_id::ObjectId;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{
    AppError,
    entities::{
        post::Model as DbPost,
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
            // 处理相对链接
            // image: self.image.and_then(|url| Some(Url::parse(&absolutize_relative_url(
            //     url,
            //     // 从 UserID 获取域名
            //     Url::parse(&self.user_id)
            //         .unwrap()
            //         .path()
            //         .split('/')
            //         .last()
            //         .unwrap()
            //         .to_string()
            // ).unwrap()).unwrap())),
            // 不处理相对链接，解析失败则视为空
            // image: self.image.and_then(|url| {
            //     if let Ok(url) = Url::parse(&url) {
            //         Some(url)
            //     } else {
            //         None
            //     }
            // }),
            // 视为字符串
            image: self.image,
            language: self.language,
            date_published: self.date_published,
            date_modified: self.date_modified,
        })
    }

    // 从 JSON 转换为本地格式
    pub async fn from_json(
        json: JsonUserFeedItem,
        user_id: ObjectId<DbUser>,
        object_id: Option<ObjectId<DbPost>>
    ) -> Result<Self, AppError> {
        Ok(Self {
            id: json.url.unwrap_or_else(|| Url::parse(&json.id).unwrap()).to_string(),
            user_id: user_id.inner().to_string(),
            object_id: object_id.map(|object_id| object_id.inner().to_string()),
            title: json.title,
            summary: json.summary,
            image: json.image.map(|url| url.to_string()),
            language: json.language,
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
    // pub image: Option<Url>,
    pub image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_published: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_modified: Option<String>,
}