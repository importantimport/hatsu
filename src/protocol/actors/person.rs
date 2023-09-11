use activitypub_federation::{
    fetch::object_id::ObjectId,
    kinds::{
        actor::PersonType,
        object::ImageType,
    },
    protocol::public_key::PublicKey,
};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::entities::user::Model as DbUser;

// ActivityPub 用户
// ActivityPub Person
// https://github.com/LemmyNet/activitypub-federation-rust/blob/main/docs/03_federating_users.md
/// https://www.w3.org/ns/activitystreams#Person
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Person {
    // 用户 ID，应为域名 + 用户名（运行时生成）
    pub id: ObjectId<DbUser>,
    // 类型，应始终为 Person
    #[serde(rename = "type")]
    pub kind: PersonType,
    // 用户名（应为域名）
    // `example.com`
    pub name: String,
    // 首选用户名（应为网站标题）
    // `Example Domain`
    pub preferred_username: String,
    // 用户描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    // 用户头像
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<PersonImage>,
    // 用户背景图
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<PersonImage>,
    // ActivityPub 用户附件（Metadata）
    // ActivityPub Person Attachment (Metadata)
    pub attachment: Vec<PersonAttachment>,
    // 收件箱
    // `https://hatsu.local/example.com/inbox`
    pub inbox: Url,
    // 发件箱
    // `https://hatsu.local/example.com/outbox`
    pub outbox: Url,
    // 关注者
    // followers: Url,
    // 正在关注
    // following: Url,
    // 公钥
    pub public_key: PublicKey,
    // TODO: bot account
    // TODO: (maybe) endpoints.sharedInbox (https://hatsu.local/inbox)
}

// ActivityPub 用户图像
// ActivityPub Person Image
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonImage {
    // 类型，应始终为 Image
    #[serde(rename = "type")]
    pub kind: ImageType,
    // 图片链接
    pub url: Url,
}

// ActivityPub 用户附件（Metadata）
// ActivityPub Person Attachment (Metadata)
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PersonAttachment {
    // 类型，应始终为 PropertyValue
    #[serde(rename = "type")]
    kind: String,
    /// Website / JSON Feed / Atom Feed / RSS Feed
    name: String,
    /// "<a href="{url}">{url}</a>"
    value: String,
}
