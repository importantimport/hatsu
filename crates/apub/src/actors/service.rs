use activitypub_federation::{
    fetch::object_id::ObjectId,
    kinds::object::ImageType,
    protocol::public_key::PublicKey,
};
// use hatsu_db_schema::user::Model as DbUser;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use url::Url;
use utoipa::ToSchema;

use crate::actors::ApubUser;

/// `ActivityPub` Service (Bot User)
/// <https://www.w3.org/ns/activitystreams#Service>
#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Service {
    // 用户 ID，应为域名 + 用户名（运行时生成）
    #[schema(value_type = Url)]
    pub id: ObjectId<ApubUser>,
    // 类型
    #[serde(rename = "type")]
    pub kind: String,
    // 用户名（应为网站标题）
    // `Example Domain`
    pub name: String,
    // 首选用户名（应为域名）
    // `example.com`
    pub preferred_username: String,
    // 用户描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    // 用户头像
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<ServiceImage>,
    // 用户背景图
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<ServiceImage>,
    // ActivityPub Service Attachment (Metadata)
    pub attachment: Vec<ServiceAttachment>,
    // 收件箱
    // `https://hatsu.local/example.com/inbox`
    pub inbox: Url,
    // 发件箱
    // `https://hatsu.local/example.com/outbox`
    pub outbox: Url,
    // 关注者
    pub followers: Url,
    // 正在关注
    pub following: Url,
    /// user name emoji
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<ServiceTag>,
    /// FEP-4adb
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<String>>,
    /// FEP-2c59
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webfinger: Option<String>,
    // 公钥
    #[schema(value_type = PublicKeySchema)]
    pub public_key: PublicKey,
}

// ActivityPub Service Image
#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ServiceImage {
    // 类型，应始终为 Image
    #[schema(value_type = String)]
    #[serde(rename = "type")]
    pub kind: ImageType,
    // 图片链接
    pub url: Url,
}

// ActivityPub Service Attachment (Metadata)
#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ServiceAttachment {
    // 类型，应始终为 PropertyValue
    #[schema(value_type = String)]
    #[serde(rename = "type")]
    kind: String,
    /// Website / JSON Feed / Atom Feed / RSS Feed
    name: String,
    /// "<a href="{url}">{url}</a>"
    value: String,
}

impl ServiceImage {
    #[must_use]
    pub const fn new(url: Url) -> Self {
        Self {
            kind: ImageType::Image,
            url,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub enum ServiceTag {
    Emoji(ServiceTagEmoji),
    Emojis(Vec<ServiceTagEmoji>),
    Any(Value),
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ServiceTagEmoji {
    #[serde(rename = "type")]
    pub kind: String, // "Emoji"
    pub icon: ServiceTagEmojiIcon,
    pub id: Url,
    pub name: String,
    pub updated: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ServiceTagEmojiIcon {
    #[schema(value_type = String)]
    #[serde(rename = "type")]
    pub kind: ImageType,
    pub media_type: String,
    pub url: Url,
}

// impl ToSchema for PublicKey
#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PublicKeySchema {
    pub id: String,
    pub owner: Url,
    pub public_key_pem: String,
}
