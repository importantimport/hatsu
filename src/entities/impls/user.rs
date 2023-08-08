use std::env;

use activitypub_federation::{
    activity_queue::send_activity,
    config::Data,
    http_signatures::generate_actor_keypair,
    protocol::verification::verify_domains_match,
    traits::{Actor, Object, ActivityHandler},
};
use chrono::{Local, NaiveDateTime};
use sea_orm::*;
use serde::Serialize;
use serde_json::to_string;
use url::Url;
// use uuid::Uuid;

use crate::{
    AppData,
    AppError,
    entities::{
        prelude::*,
        activity::{self, Model as DbActivity},
        user::{self, Model as DbUser}
    },
    protocol::actors::Person,
    utilities::get_site_feed,
};

impl DbUser {
  // 创建新用户
  // Create a new user
  // TODO: 从网站获取数据
  // TODO: Getting data from websites
  pub async fn new(preferred_username: &str) -> Result<Self, AppError> {
      let hostname = env::var("HATSU_DOMAIN")?;
      let id = Url::parse(&format!("https://{}/u/{}", hostname, &preferred_username))?;
      let inbox = Url::parse(&format!("https://{}/u/{}/inbox", hostname, &preferred_username))?;
      let outbox = Url::parse(&format!("https://{}/u/{}/outbox", hostname, &preferred_username))?;
      let keypair = generate_actor_keypair()?;

      let feed = get_site_feed(preferred_username.to_string()).await?;

      tracing::info!(
          "User Feed: {}, {}, {}",
          feed.json.unwrap_or_else(|| "null".to_string()),
          feed.atom.unwrap_or_else(|| "null".to_string()),
          feed.rss.unwrap_or_else(|| "null".to_string()),
      );

      Ok(Self {
          id: id.to_string(),
          name: "Hatsu".to_string(),
          preferred_username: preferred_username.to_string(),
          inbox: inbox.to_string(),
          outbox: outbox.to_string(),
          local: true,
          public_key: keypair.public_key,
          private_key: Some(keypair.private_key),
          last_refreshed_at: Local::now().naive_local().format("%Y-%m-%d %H:%M:%S").to_string(),
          // followers: vec![],
      })
  }

  /// 发送动态 / Send Activity
  /// 
  /// activitypub_federation::activity_queue::send_activity 的简单封装
  /// 
  /// 遇到类型问题加不上去，不要忘了用 WithContext::new_default(activity) 套一层
  /// 
  /// https://github.com/LemmyNet/activitypub-federation-rust/blob/35bf29ae73e33a537a9fdb2d2bb8bb1ba4842991/examples/federation/objects/person.rs#L111-L132
  pub async fn send<Activity>(
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
    let activity_id: String = activity
        .id()
        .path()
        .split('/')
        .last()
        .unwrap()
        .to_string();

    // 验证这个 UUID
    // let uuid = Uuid::try_parse(&activity_id)?;

    // 保存到数据库
    activity::Entity::insert(DbActivity {
        id: activity_id,
        activity: to_string(&activity)?
    }.into_active_model())
        .exec(&data.conn)
        .await?;

    // let activity = WithContext::new_default(activity);
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
        Some(NaiveDateTime::parse_from_str(&self.last_refreshed_at, "%Y-%m-%d %H:%M:%S").unwrap())
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
            name: self.name.clone(),
            preferred_username: self.preferred_username.clone(),
            kind: Default::default(),
            id: Url::parse(&self.id).unwrap().into(),
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
            inbox: json.inbox.to_string(),
            outbox: json.outbox.to_string(),
            public_key: json.public_key.public_key_pem,
            private_key: None,
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
