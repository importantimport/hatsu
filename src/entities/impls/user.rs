use std::env;

use activitypub_federation::{
    activity_queue::send_activity,
    config::Data,
    http_signatures::generate_actor_keypair,
    protocol::verification::verify_domains_match,
    traits::{Actor, Object, ActivityHandler},
};
use chrono::{DateTime, Local, NaiveDateTime, SecondsFormat};
use sea_orm::*;
use serde::Serialize;
use url::Url;
// use uuid::Uuid;

use crate::{
    AppData,
    AppError,
    entities::{
        prelude::*,
        impls::JsonUserFeed,
        user::{self, Model as DbUser},
    },
    protocol::actors::{Person, PersonImage},
    utilities::get_site_feed,
};

impl DbUser {
    // 创建新用户
    // Create a new user
    pub async fn new(preferred_username: &str) -> Result<Self, AppError> {
        // TODO: data.domain()
        let hostname = env::var("HATSU_DOMAIN")?;
        let id = Url::parse(&format!("https://{}/u/{}", hostname, &preferred_username))?;
        let inbox = Url::parse(&format!("https://{}/u/{}/inbox", hostname, &preferred_username))?;
        let outbox = Url::parse(&format!("https://{}/u/{}/outbox", hostname, &preferred_username))?;
        let keypair = generate_actor_keypair()?;

        let feed = get_site_feed(preferred_username.to_string()).await?;

        // Tests for JSON Feed only
        let json_feed: JsonUserFeed = reqwest::get(Url::parse(&feed.json.clone().unwrap())?)
            .await?
            .json::<JsonUserFeed>()
            .await?;

        let user = Self {
            id: id.to_string(),
            name: json_feed.title,
            preferred_username: preferred_username.to_string(),
            summary: json_feed.description,
            icon: json_feed.icon.map(|url| url.to_string()),
            image: json_feed.hatsu.and_then(|hatsu| hatsu.banner_image.map(|url| url.to_string())),
            inbox: inbox.to_string(),
            outbox: outbox.to_string(),
            local: true,
            public_key: keypair.public_key,
            private_key: Some(keypair.private_key),
            feed_json: feed.json,
            feed_atom: feed.atom,
            feed_rss: feed.rss,
            last_refreshed_at: Local::now().to_rfc3339_opts(SecondsFormat::Secs, true),
            // followers: vec![],
        };

        Ok(user)
    }

  /// 发送动态 / Send Activity
  /// 
  /// activitypub_federation::activity_queue::send_activity 的简单封装
  /// 
  /// 遇到类型问题加不上去，不要忘了用 WithContext::new_default(activity) 套一层
  /// 
  /// https://github.com/LemmyNet/activitypub-federation-rust/blob/35bf29ae73e33a537a9fdb2d2bb8bb1ba4842991/examples/federation/objects/person.rs#L111-L132
  pub async fn send_activity<Activity>(
    &self,
    activity: Activity,
    inboxes: Vec<Url>,
    data: &Data<AppData>,
  ) -> Result<(), <Activity as ActivityHandler>::Error>
  where
    Activity: ActivityHandler + Serialize,
    <Activity as ActivityHandler>::Error: From<anyhow::Error> + From<serde_json::Error> + From<migration::DbErr>
  {
    // 从 Activity URL 提取 UUID
    // let activity_id: String = activity
    //     .id()
    //     .path()
    //     .split('/')
    //     .last()
    //     .unwrap()
    //     .to_string();

    // 验证这个 UUID
    // let uuid = Uuid::try_parse(&activity_id)?;

    // 保存到数据库
    // activity::Entity::insert(DbActivity {
    //     id: activity_id,
    //     activity: to_string(&activity)?,
    //     actor: activity.actor().to_string(),
    //     kind: activity.kind,

    // }.into_active_model())
    //     .exec(&data.conn)
    //     .await?;

    // 发送
    send_activity(activity, self, inboxes, data).await?;
    Ok(())
  }
}

#[async_trait::async_trait]
impl Object for DbUser {
    type DataType = AppData;
    type Kind = Person;
    type Error = AppError;

    fn last_refreshed_at(&self) -> Option<NaiveDateTime> {
        Some(DateTime::parse_from_rfc3339(&self.last_refreshed_at).unwrap().naive_local())
    }

    // 从 ID 读取
    async fn read_from_id(
        object_id: Url,
        data: &Data<Self::DataType>,
    ) -> Result<Option<Self>, Self::Error> {
        Ok(User::find_by_id(&object_id.to_string())
            .one(&data.conn)
            .await?)
    }

    // 转换为 ActivityStreams JSON
    async fn into_json(self, _data: &Data<Self::DataType>) -> Result<Self::Kind, Self::Error> {
        Ok(Person {
            kind: Default::default(),
            name: self.name.clone(),
            preferred_username: self.preferred_username.clone(),
            id: Url::parse(&self.id).unwrap().into(),
            summary: self.summary.clone(),
            icon: self.icon.clone().map(|icon| PersonImage {
                kind: Default::default(),
                url: Url::parse(&icon).unwrap()
            }),
            image: self.image.clone().map(|image| PersonImage {
                kind: Default::default(),
                url: Url::parse(&image).unwrap()
            }),
            // TODO: User Attachment
            attachment: vec![],
            inbox: Url::parse(&self.inbox)?,
            outbox: Url::parse(&self.outbox)?,
            public_key: self.public_key(),
        })
    }

    // 验证
    async fn verify(
        json: &Self::Kind,
        expected_domain: &Url,
        _data: &Data<Self::DataType>,
    ) -> Result<(), Self::Error> {
        verify_domains_match(json.id.inner(), expected_domain)?;
        Ok(())
    }

    // 转换为本地格式（同时保存到数据库）
    async fn from_json(
        json: Self::Kind,
        data: &Data<Self::DataType>,
    ) -> Result<Self, Self::Error> {
        let user = Self {
            id: json.id.to_string(),
            name: json.name,
            preferred_username: json.preferred_username,
            summary: json.summary,
            icon: json.icon.map(|icon| icon.url.to_string()),
            image: json.image.map(|image| image.url.to_string()),
            inbox: json.inbox.to_string(),
            outbox: json.outbox.to_string(),
            public_key: json.public_key.public_key_pem,
            private_key: None,
            feed_json: None,
            feed_atom: None,
            feed_rss: None,
            last_refreshed_at: Local::now().naive_local().format("%Y-%m-%d %H:%M:%S").to_string(),
            // followers: vec![],
            local: false,
        };

        // 写入数据库
        // TODO: on_conflict 时执行更新
        User::insert(user.clone().into_active_model())
            .on_conflict(
                sea_query::OnConflict::column(user::Column::Id)
                    .do_nothing()
                    .to_owned()
            )
            .do_nothing()
            .exec(&data.conn)
            .await?;

        Ok(user)
    }

    // 删除用户
    async fn delete(self, data: &Data<Self::DataType>) -> Result<(), Self::Error> {
        let _delete_user = User::delete_by_id(&self.id.to_string())
            .exec(&data.conn)
            .await?;
        Ok(())
    }
}

impl Actor for DbUser {
    fn id(&self) -> Url {
        Url::parse(&self.id).unwrap()
    }

    fn public_key_pem(&self) -> &str {
        &self.public_key
    }

    fn private_key_pem(&self) -> Option<String> {
        self.private_key.clone()
    }

    fn inbox(&self) -> Url {
        Url::parse(&self.inbox).unwrap()
    }
}
