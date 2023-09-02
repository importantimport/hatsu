use activitypub_federation::{
    config::Data,
    fetch::object_id::ObjectId,
    kinds::activity::FollowType,
    protocol::helpers::deserialize_skip_error,
    traits::{Actor, ActivityHandler},
};
use sea_orm::*;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{
    AppData,
    AppError,
    entities::{
        received_follow::Model as DbReceivedFollow,
        user::Model as DbUser,
    },
    protocol::activities::AcceptFollow,
};

/// https://github.com/LemmyNet/lemmy/blob/963d04b3526f8a5e9ff762960bfb5215e353bb27/crates/apub/src/protocol/activities/following/follow.rs
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Follow {
    /// 关注者
    pub(crate) actor: ObjectId<DbUser>,
    /// Optional, for compatibility with platforms that always expect recipient field
    #[serde(deserialize_with = "deserialize_skip_error", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) to: Option<[ObjectId<DbUser>; 1]>,
    /// 被关注者
    pub(crate) object: ObjectId<DbUser>,
    #[serde(rename = "type")]
    pub(crate) kind: FollowType,
    pub(crate) id: Url,
}

/// 只接收，不发送
/// receive only, without send
/// https://github.com/LemmyNet/lemmy/blob/963d04b3526f8a5e9ff762960bfb5215e353bb27/crates/apub/src/activities/following/follow.rs
#[async_trait::async_trait]
impl ActivityHandler for Follow {
    type DataType = AppData;
    type Error = AppError;

    fn id(&self) -> &Url {
        &self.id
    }

    fn actor(&self) -> &Url {
        self.actor.inner()
    }

    async fn verify(&self, _data: &Data<Self::DataType>) -> Result<(), Self::Error> {
        // TODO
        Ok(())
    }

    async fn receive(self, data: &Data<Self::DataType>) -> Result<(), Self::Error> {
        // 被关注者（本地账号）, user
        let object = self.object.dereference_local(data).await?;
        // 关注者, follower
        let actor = self.actor.dereference(data).await?;

        // 添加关注记录
        let _insert_follow = DbReceivedFollow::from_json(self.clone())?
            .into_active_model()
            .insert(&data.conn)
            .await;

        // 发送接受关注
        object.send_activity(AcceptFollow::new(self, data).await?, vec![actor.shared_inbox_or_inbox()], data).await?;

        Ok(())
    }
}