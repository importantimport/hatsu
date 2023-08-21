use activitypub_federation::fetch::object_id::ObjectId;
use serde::{Deserialize, Serialize};

use crate::{
    AppError,
    entities::{
        user::Model as DbUser,
        user_feed::Model as DbUserFeed,
    }
};

impl DbUserFeed {
    // 转换为 JSON
    async fn into_json(self) -> Result<UserFeed, AppError> {
        Ok(UserFeed {
            hatsu: match self.hatsu {
                Some(hatsu) => Some(serde_json::from_str(&hatsu)?),
                None => None,
            },
            feed_url: self.feed_url,
            next_url: self.next_url,
            title: self.title,
            description: self.description,
            icon: self.icon,
            language: self.language,
            items: serde_json::from_str(&self.items)?
        })
    }

    // 从 JSON 转换为本地格式
    async fn from_json(
        json: UserFeed,
        user_id: ObjectId<DbUser>
    ) -> Result<Self, AppError> {
        Ok(Self {
            user_id: user_id.inner().to_string(),
            hatsu: match json.hatsu {
                Some(hatsu) => Some(serde_json::to_string(&hatsu)?),
                None => None,
            },
            feed_url: json.feed_url,
            next_url: json.next_url,
            title: json.title,
            description: json.description,
            icon: json.icon,
            language: json.language,
            items: serde_json::to_string(&json.items)?
        })
    }

    // 从字符串转换为本地格式
    async fn from_str(
        str: String,
        user_id: ObjectId<DbUser>
    ) -> Result<Self, AppError> {
        let json: UserFeed = serde_json::from_str(&str)?;

        Ok(Self {
            user_id: user_id.inner().to_string(),
            hatsu: match json.hatsu {
                Some(hatsu) => Some(serde_json::to_string(&hatsu)?),
                None => None,
            },
            feed_url: json.feed_url,
            next_url: json.next_url,
            title: json.title,
            description: json.description,
            icon: json.icon,
            language: json.language,
            items: serde_json::to_string(&json.items)?
        })
    }
}

/// JSON Feed 1.1
/// 
/// https://www.jsonfeed.org/version/1.1/#top-level-a-name-top-level-a
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserFeed {
    // pub user_id: String,
    #[serde(rename = "_hatsu")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hatsu: Option<UserFeedHatsu>,
    pub feed_url: String,
    pub next_url: Option<String>,
    pub title: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub language: Option<String>,
    pub items: Vec<UserFeedItem>,
}

/// Hatsu JSON Feed Extension
/// 
/// https://github.com/importantimport/hatsu/issues/1
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserFeedHatsu {
    about: Option<String>,
    banner_image: Option<String>,
}

/// JSON Feed Items
/// 
/// https://www.jsonfeed.org/version/1.1/#items-a-name-items-a
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserFeedItem {
    id: String,
    url: Option<String>,
    title: Option<String>,
    content_html: Option<String>,
    content_text: Option<String>,
    summary: Option<String>,
    image: Option<String>,
    date_published: Option<String>,
    date_modified: Option<String>,
    language: Option<String>,
}