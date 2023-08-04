use activitypub_federation::{
    config::Data,
    fetch::object_id::ObjectId,
    kinds::activity::FollowType,
    protocol::helpers::deserialize_skip_error,
    traits::ActivityHandler,
};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{
    AppData,
    AppError,
    entities::{
        user::Model as DbUser,
        user_follower::Model as DbUserFollower
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

        // TODO: 验证可用性
        DbUserFollower::new(
            Url::parse(&object.id)?,
            Url::parse(&actor.id)?,
            data
        ).await?;

        // TODO: 验证可用性
        AcceptFollow::send(self, data).await?;

        Ok(())
    }
}