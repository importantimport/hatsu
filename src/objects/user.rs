use activitypub_federation::{
    config::Data,
    fetch::object_id::ObjectId,
    kinds::actor::PersonType,
    protocol::public_key::PublicKey,
    traits::ActivityHandler,
};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{
    activities::create_post::CreatePost,
    entities::user::Model as DbUser,
};

// ActivityPub 用户
// ActivityPub Person
// https://github.com/LemmyNet/activitypub-federation-rust/blob/main/docs/03_federating_users.md
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
    // summary: Option<String>,
    // 用户头像
    // icon: Option<PersonImage>,
    // 用户背景图
    // image: Option<PersonImage>,
    // 收件箱
    // `https://hatsu.local/example.com/inbox`
    pub inbox: Url,
    // 发件箱
    // `https://hatsu.local/example.com/outbox`
    pub outbox: Url,
    // 公钥
    pub public_key: PublicKey,
    // ActivityPub 用户附件（Metadata）
    // ActivityPub Person Attachment (Metadata)
    // attachment: Vec<PersonAttachment>,
    // 关注者
    // followers: Url,
    // 正在关注
    // following: Url,
}

// ActivityPub 用户图像
// ActivityPub Person Image
// #[derive(Clone, Debug, Deserialize, Serialize)]
// #[serde(rename_all = "camelCase")]
// pub struct PersonImage {
//     // 类型，应始终为 Image
//     #[serde(rename = "type")]
//     kind: String,
//     // 图片链接
//     url: String,
// }

// ActivityPub 用户附件（Metadata）
// ActivityPub Person Attachment (Metadata)
// #[derive(Clone, Debug, Deserialize, Serialize)]
// #[serde(rename_all = "camelCase")]
// pub struct PersonAttachment {
//     // 类型，应始终为 PropertyValue
//     #[serde(rename = "type")]
//     kind: String,
//     name: String,
//     value: String,
// }

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
#[enum_delegate::implement(ActivityHandler)]
pub enum PersonAcceptedActivities {
    CreateNote(CreatePost)
}

// 数据库用户 Feed
// Database User Feed
// #[derive(Clone, Debug)]
// pub struct DbUserFeed {
//     // json / atom / rss
//     name: String,
//     value: Url,
// }
