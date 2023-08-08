// https://github.com/LemmyNet/activitypub-federation-rust/blob/61085a643f05dbb70502b3c519fd666214b7e308/examples/live_federation/objects/post.rs
// https://github.com/LemmyNet/lemmy/blob/main/crates/apub/assets

use std::env;

use activitypub_federation::{
    config::Data,
    fetch::object_id::ObjectId,
    kinds::public,
    protocol::verification::verify_domains_match,
    traits::{Actor, Object},
};
use sea_orm::*;
use url::Url;
use uuid::Uuid;

use crate::{
    AppData,
    AppError,
    protocol::{
      activities::create_post::CreatePost,
      links::Mention,
      objects::Note,
    },
    entities::{
        prelude::*,
        post::Model as DbPost,
        user::Model as DbUser,
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
    async fn into_json(self, data: &Data<Self::DataType>) -> Result<Self::Kind, Self::Error> {
        // TODO: 不确定是否可用
        let object_id: ObjectId<DbUser> = Url::parse(&self.creator)?.into();
        let creator = object_id.dereference_local(data).await?;
        let mention = Mention {
            href: Url::parse(&creator.id)?,
            kind: Default::default()
        };
        let note = Note {
            kind: Default::default(),
            id: Url::parse(&self.id)?.into(),
            attributed_to: Url::parse(&self.creator)?.into(),
            // TODO:
            // to: vec![public(), creator.followers_url()?],
            to: vec![public()],
            cc: vec![],
            content: self.text,
            in_reply_to: None,
            tag: vec![mention]
        };

        Ok(note)
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

        let creator = json.attributed_to.dereference(data).await?;
        let post = DbPost {
            id: json.id.to_string(),
            creator: json.attributed_to.to_string(),
            text: json.content,
            local: false,
        };

        let mention = Mention {
            href: Url::parse(&creator.id)?,
            kind: Default::default()
        };
        let note = Note {
            kind: Default::default(),
            id: Url::parse(&format!("https://{}/o/{}", data.domain(), Uuid::now_v7()))?.into(),
            // TODO: multiple user
            attributed_to: Url::parse(&format!("https://{}/u/{}", data.domain(), env::var("HATSU_TEST_ACCOUNT")?))?.into(),
            // 发送给提及的用户
            // TODO: "cc": ["https://{}/u/{}/followers"]
            to: vec![public(), json.attributed_to.clone().into()],
            cc: vec![],
            content: format!("Hello {}", creator.name),
            in_reply_to: Some(json.id.clone()),
            tag: vec![mention]
        };

        // DEBUG TEST: 保存到数据库 / Save Note to Database
        let db_post = DbPost {
            id: note.id.to_string(),
            creator: note.attributed_to.to_string(),
            text: note.content.clone(),
            local: true,
        }.into_active_model();

        // DEBUG TEST: 保存到数据库 / Save Note to Database
        let _insert_db_post = Post::insert(db_post)
            .exec(&data.conn)
            .await?;

        CreatePost::send(note, creator.shared_inbox_or_inbox(), data).await?;

        Ok(post)
    }

    // 删除帖文
    async fn delete(self, data: &Data<Self::DataType>) -> Result<(), Self::Error> {
        let _delete_post = Post::delete_by_id(&self.id.to_string())
            .exec(&data.conn)
            .await?;
        Ok(())
    }

    // fn last_refreshed_at(&self) -> Option<chrono::NaiveDateTime> {
    //     todo!()
    // }
}