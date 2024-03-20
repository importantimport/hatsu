// https://github.com/LemmyNet/activitypub-federation-rust/blob/61085a643f05dbb70502b3c519fd666214b7e308/examples/live_federation/objects/post.rs
// https://github.com/LemmyNet/lemmy/blob/main/crates/apub/assets

use std::ops::Deref;

use activitypub_federation::{
    config::Data,
    kinds::public,
    protocol::verification::verify_domains_match,
    traits::Object,
};
use axum::http::StatusCode;
use chrono::{DateTime, Utc};
use hatsu_db_schema::{post::Model as DbPost, prelude::Post};
use hatsu_utils::{AppData, AppError};
use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel};
use url::Url;

use crate::objects::Note;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ApubPost(pub(crate) DbPost);

impl AsRef<DbPost> for ApubPost {
    fn as_ref(&self) -> &DbPost {
        &self.0
    }
}

impl Deref for ApubPost {
    type Target = DbPost;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<DbPost> for ApubPost {
    fn from(p: DbPost) -> Self {
        Self(p)
    }
}

#[async_trait::async_trait]
impl Object for ApubPost {
    type DataType = AppData;
    type Error = AppError;
    type Kind = Note;

    // 从 ID 读取
    async fn read_from_id(
        post_id: Url,
        data: &Data<Self::DataType>,
    ) -> Result<Option<Self>, Self::Error> {
        Ok(Post::find_by_id(&post_id.to_string())
            .one(&data.conn)
            .await?
            .map(Into::into))
    }

    // 转换为 ActivityStreams JSON
    async fn into_json(self, _data: &Data<Self::DataType>) -> Result<Self::Kind, Self::Error> {
        Ok(serde_json::from_str(&self.object)?)
    }

    // 验证
    async fn verify(
        json: &Self::Kind,
        expected_domain: &Url,
        _data: &Data<Self::DataType>,
    ) -> Result<(), Self::Error> {
        verify_domains_match(json.id.inner(), expected_domain)?;

        // https://github.com/LemmyNet/lemmy/blob/2fd81067c7130a23cabfff8bfa76b349b87e8426/crates/apub/src/activities/mod.rs#L127-L133
        if [json.to.as_slice(), json.cc.as_slice()]
            .concat()
            .contains(&public())
        {
            Ok(())
        } else {
            Err(AppError::new(
                format!("Post is not Public: {}", json.id),
                None,
                Some(StatusCode::BAD_REQUEST),
            ))
        }
    }

    // 转换为本地格式
    async fn from_json(json: Self::Kind, data: &Data<Self::DataType>) -> Result<Self, Self::Error> {
        tracing::info!(
            "Received post with content {} and id {}",
            &json.content,
            &json.id
        );

        let note = json.clone();

        // let creator = json.attributed_to.dereference(data).await?;
        // 转换为数据库格式并保存到数据库
        let post = DbPost {
            id: json.id.to_string(),
            attributed_to: json.attributed_to.to_string(),
            object: serde_json::to_string(&json)?,
            published: json.published,
            updated: json.updated,
            in_reply_to: json.in_reply_to.map(|url| url.to_string()),
            in_reply_to_root: note.check_in_reply_to_root(data).await?,
            last_refreshed_at: hatsu_utils::date::now(),
            local: false,
        }
        .into_active_model()
        .insert(&data.conn)
        .await?;

        Ok(post.into())
    }

    // 删除帖文
    async fn delete(self, data: &Data<Self::DataType>) -> Result<(), Self::Error> {
        let _delete_post = Post::delete_by_id(&self.id.to_string())
            .exec(&data.conn)
            .await?;
        Ok(())
    }

    fn last_refreshed_at(&self) -> Option<DateTime<Utc>> {
        hatsu_utils::date::parse(&self.last_refreshed_at).ok()
    }
}
