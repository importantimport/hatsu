use activitypub_federation::{
  config::Data,
  fetch::object_id::ObjectId,
  kinds::activity::UndoType,
  protocol::helpers::deserialize_skip_error,
  traits::ActivityHandler,
};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{
  AppData,
  protocol::activities::following::follow::Follow,
  error::AppError,
  entities::user::Model as DbUser,
};

/// https://github.com/LemmyNet/lemmy/blob/963d04b3526f8a5e9ff762960bfb5215e353bb27/crates/apub/src/protocol/activities/following/undo_follow.rs
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UndoFollow {
  pub(crate) actor: ObjectId<DbUser>,
  /// Optional, for compatibility with platforms that always expect recipient field
  #[serde(deserialize_with = "deserialize_skip_error", default)]
  pub(crate) to: Option<[ObjectId<DbUser>; 1]>,
  pub(crate) object: Follow,
  #[serde(rename = "type")]
  pub(crate) kind: UndoType,
  pub(crate) id: Url,
}

/// 只接收，不发送
/// receive only, without send
/// https://github.com/LemmyNet/lemmy/blob/963d04b3526f8a5e9ff762960bfb5215e353bb27/crates/apub/src/activities/following/undo_follow.rs
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

  async fn receive(self, _data: &Data<Self::DataType>) -> Result<(), Self::Error> {
      // TODO
      Ok(())
  }
}