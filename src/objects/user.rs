use activitypub_federation::{
    config::Data,
    fetch::object_id::ObjectId,
    http_signatures::generate_actor_keypair,
    kinds::actor::PersonType,
    protocol::{public_key::PublicKey, verification::verify_domains_match},
    traits::{Actor, Object},
};
use chrono::{Local, NaiveDateTime};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{database::Database, error::Error};

// ActivityPub 用户
// ActivityPub Person
// https://github.com/LemmyNet/activitypub-federation-rust/blob/main/docs/03_federating_users.md
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Person {
    // 用户 ID，应为域名 + 用户名（运行时生成）
    id: ObjectId<DbUser>,
    // 类型，应始终为 Person
    #[serde(rename = "type")]
    kind: PersonType,
    // 用户名（应为域名）
    // `example.com`
    name: String,
    // 首选用户名（应为网站标题）
    // `Example Domain`
    preferred_username: Option<String>,
    // 用户描述
    // summary: Option<String>,
    // 用户头像
    // icon: Option<PersonImage>,
    // 用户背景图
    // image: Option<PersonImage>,
    // 收件箱
    // `https://hatsu.local/example.com/inbox`
    inbox: Url,
    // 发件箱
    // `https://hatsu.local/example.com/outbox`
    outbox: Url,
    // 公钥
    public_key: PublicKey,
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

// 数据库用户
// Database User
// https://github.com/LemmyNet/activitypub-federation-rust/blob/main/docs/03_federating_users.md
#[derive(Debug, Clone)]
pub struct DbUser {
    // 用户 ID，应为域名 + 用户名
    pub id: ObjectId<DbUser>,
    // 用户名
    pub name: String,
    // 首选用户名（应为网站标题）
    // `Example Domain`
    pub preferred_username: Option<String>,
    // 用户收件箱
    pub inbox: Url,
    // 用户发件箱
    pub outbox: Url,
    // 是否为本地用户
    pub local: bool,
    // 用户公钥，存在于所有用户（验证 HTTP 签名所必需）
    // user public key, exists for all users (necessary to verify http signatures)
    pub public_key: String,
    // 用户私钥，仅存在于本地用户
    // user private key, exists only for local users
    pub private_key: Option<String>,
    // 最后更新时间
    last_refreshed_at: NaiveDateTime,
    // pub followers: Vec<Url>,
}

// 数据库用户 Feed
// Database User Feed
// #[derive(Clone, Debug)]
// pub struct DbUserFeed {
//     // json / atom / rss
//     name: String,
//     value: Url,
// }

impl DbUser {
    // 创建新用户
    // Create a new user
    // TODO: 从网站获取数据
    // TODO: Getting data from websites
    pub fn new(hostname: &str, name: &str) -> Result<Self, Error> {
        // let ap_id = Url::parse(&format!("https://{}/{}", hostname, &name))?.into();
        let id = Url::parse(&format!("https://{}/{}", hostname, &name))?.into();
        let inbox = Url::parse(&format!("https://{}/{}/inbox", hostname, &name))?;
        let outbox = Url::parse(&format!("https://{}/{}/outbox", hostname, &name))?;
        let keypair = generate_actor_keypair()?;

        Ok(Self {
            id,
            name: name.to_string(),
            preferred_username: Some("Hatsu".to_string()),
            inbox,
            outbox,
            local: true,
            public_key: keypair.public_key,
            private_key: Some(keypair.private_key),
            last_refreshed_at: Local::now().naive_local(),
            // followers: vec![],
        })
    }
}

#[async_trait::async_trait]
impl Object for DbUser {
    type DataType = Database;
    type Kind = Person;
    type Error = Error;

    fn last_refreshed_at(&self) -> Option<NaiveDateTime> {
        Some(self.last_refreshed_at)
    }

    async fn read_from_id(
        _object_id: Url,
        _data: &Data<Self::DataType>,
    ) -> Result<Option<Self>, Self::Error> {
        // let users = data.users.lock().unwrap();
        // let res = users
        //     .clone()
        //     .into_iter()
        //     .find(|u| u.ap_id.inner() == &object_id);
        // Ok(res)
        Ok(None)
    }

    // 转换为 ActivityStreams JSON
    // Convert to ActivityStreams JSON
    async fn into_json(self, _data: &Data<Self::DataType>) -> Result<Self::Kind, Self::Error> {
        Ok(Person {
            name: self.name.clone(),
            preferred_username: self.preferred_username.clone(),
            kind: Default::default(),
            id: self.id.clone(),
            inbox: self.inbox.clone(),
            outbox: self.outbox.clone(),
            public_key: self.public_key(),
        })
    }

    async fn verify(
        json: &Self::Kind,
        expected_domain: &Url,
        _data: &Data<Self::DataType>,
    ) -> Result<(), Self::Error> {
        verify_domains_match(json.id.inner(), expected_domain)?;
        Ok(())
    }

    async fn from_json(
        json: Self::Kind,
        _data: &Data<Self::DataType>,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            id: json.id,
            name: json.name,
            preferred_username: json.preferred_username,
            inbox: json.inbox,
            outbox: json.outbox,
            public_key: json.public_key.public_key_pem,
            private_key: None,
            last_refreshed_at: Local::now().naive_local(),
            // followers: vec![],
            local: false,
        })
    }

    async fn delete(self, _data: &Data<Self::DataType>) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl Actor for DbUser {
    fn id(&self) -> Url {
        self.id.inner().clone()
    }

    fn public_key_pem(&self) -> &str {
        &self.public_key
    }

    fn private_key_pem(&self) -> Option<String> {
        self.private_key.clone()
    }

    fn inbox(&self) -> Url {
        self.inbox.clone()
    }
}
