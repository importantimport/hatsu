// https://github.com/LemmyNet/activitypub-federation-rust/blob/61085a643f05dbb70502b3c519fd666214b7e308/examples/live_federation/objects/post.rs
// https://github.com/LemmyNet/lemmy/blob/main/crates/apub/assets

// use std::env;

use activitypub_federation::{
    config::Data,
    // kinds::public,
    protocol::verification::verify_domains_match,
    traits::Object,
    // traits::{Actor, Object},
};
use chrono::{DateTime, Local, NaiveDateTime, SecondsFormat};
use sea_orm::*;
use url::Url;
// use uuid::Uuid;

use crate::{
    AppData,
    AppError,
    protocol::{
    //   activities::{CreateOrUpdateNote, CreateOrUpdateType},
    //   links::Mention,
      objects::Note,
    },
    entities::{
        prelude::*,
        post::Model as DbPost,
    },
};

#[async_trait::async_trait]
impl Object for DbPost {
    type DataType = AppData;
    type Kind = Note;
    type Error = AppError;

    // 从 ID 读取
    async fn read_from_id(
        object_id: Url,
        data: &Data<Self::DataType>
    ) -> Result<Option<Self>, Self::Error> {
        Ok(Post::find_by_id(&object_id.to_string())
            .one(&data.conn)
            .await?)
    }

    // 转换为 ActivityStreams JSON
    async fn into_json(self, _data: &Data<Self::DataType>) -> Result<Self::Kind, Self::Error> {
        Ok(serde_json::from_str(&self.object)?)
    }

    // 验证
    async fn verify(
        json: &Self::Kind,
        expected_domain: &Url,
        _data: &Data<Self::DataType>
    ) -> Result<(), Self::Error> {
        verify_domains_match(json.id.inner(), expected_domain)?;
        Ok(())
    }

    // 转换为本地格式
    async fn from_json(json: Self::Kind, data: &Data<Self::DataType>) -> Result<Self, Self::Error> {
        tracing::info!("Received post with content {} and id {}", &json.content, &json.id);

        // let creator = json.attributed_to.dereference(data).await?;
        // 转换为数据库格式并保存到数据库
        // TODO: 存放到 received_note
        let post = Self {
            id: json.id.to_string(),
            attributed_to: json.attributed_to.to_string(),
            object: serde_json::to_string(&json)?,
            published: json.published,
            updated: json.updated,
            last_refreshed_at: Local::now().to_rfc3339_opts(SecondsFormat::Secs, true),
            local: false,
        }.into_active_model().insert(&data.conn).await?;

        // let mention = Mention {
        //     href: Url::parse(&creator.id)?,
        //     kind: Default::default()
        // };
        // let note = Note {
        //     kind: Default::default(),
        //     id: Url::parse(&format!("https://{}/o/{}", data.domain(), Uuid::now_v7()))?.into(),
        //     // TODO: multiple user / 多用户
        //     attributed_to: Url::parse(&format!("https://{}/u/{}", data.domain(), env::var("HATSU_PRIMARY_ACCOUNT")?))?.into(),
        //     // 发送给提及的用户
        //     // TODO: "to": ["https://{}/u/{}/followers"]
        //     to: vec![json.attributed_to.clone().into()],
        //     cc: vec![public()],
        //     source: "".to_string(),
        //     content: format!("Hello {}", creator.name),
        //     in_reply_to: Some(json.id.clone()),
        //     tag: Some(vec![mention]),
        //     published: Some(Local::now().to_rfc3339_opts(SecondsFormat::Secs, true)),
        //     updated: None,
        // };

        // 保存到数据库 / Save Note to Database
        // let _insert_post = DbPost {
        //     id: note.id.to_string(),
        //     attributed_to: note.attributed_to.to_string(),
        //     object: serde_json::to_string(&note)?,
        //     published: note.published.clone(),
        //     updated: note.updated.clone(),
        //     last_refreshed_at: note.published.clone().unwrap(),
        //     local: true,
        // }
        //     .into_active_model()
        //     .insert(&data.conn)
        //     .await?;

        // // 获取本地用户
        // let person = note.attributed_to.dereference_local(data).await?;

        // // Send Activity
        // person.send_activity(CreateOrUpdateNote::new(note, CreateOrUpdateType::Create, data).await?, vec![creator.shared_inbox_or_inbox()], data).await?;

        Ok(post)
    }

    // 删除帖文
    async fn delete(self, data: &Data<Self::DataType>) -> Result<(), Self::Error> {
        let _delete_post = Post::delete_by_id(&self.id.to_string())
            .exec(&data.conn)
            .await?;
        Ok(())
    }

    fn last_refreshed_at(&self) -> Option<NaiveDateTime> {
        Some(DateTime::parse_from_rfc3339(&self.last_refreshed_at).unwrap().naive_local())
    }
}
