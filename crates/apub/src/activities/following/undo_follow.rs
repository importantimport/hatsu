use activitypub_federation::{
    config::Data,
    fetch::object_id::ObjectId,
    kinds::activity::UndoType,
    protocol::helpers::deserialize_skip_error,
    traits::ActivityHandler,
};
use hatsu_db_schema::prelude::ReceivedFollow;
use hatsu_utils::{AppData, AppError};
use sea_orm::EntityTrait;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{activities::Follow, actors::ApubUser};

// https://github.com/LemmyNet/lemmy/blob/963d04b3526f8a5e9ff762960bfb5215e353bb27/crates/apub/src/protocol/activities/following/undo_follow.rs
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UndoFollow {
    pub(crate) actor: ObjectId<ApubUser>,
    /// Optional, for compatibility with platforms that always expect recipient field
    #[serde(deserialize_with = "deserialize_skip_error", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) to: Option<[ObjectId<ApubUser>; 1]>,
    pub(crate) object: Follow,
    #[serde(rename = "type")]
    pub(crate) kind: UndoType,
    pub(crate) id: Url,
}

/// 只接收，不发送
/// receive only, without send
/// <https://github.com/LemmyNet/lemmy/blob/963d04b3526f8a5e9ff762960bfb5215e353bb27/crates/apub/src/activities/following/undo_follow.rs>
#[async_trait::async_trait]
impl ActivityHandler for UndoFollow {
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
        // 被取消关注者（本地账号）, user
        // let object = self.object.object.dereference_local(data).await?;
        // 取消关注者, unfollower
        // let actor = self.actor.dereference(data).await?;

        // 删除关注记录
        ReceivedFollow::delete_by_id(self.object.id)
            .exec(&data.conn)
            .await?;

        Ok(())
    }
}
