// https://github.com/LemmyNet/activitypub-federation-rust/blob/61085a643f05dbb70502b3c519fd666214b7e308/examples/live_federation/activities/create_post.rs

use activitypub_federation::{
  config::Data,
  fetch::object_id::ObjectId,
  kinds::activity::CreateType,
  protocol::helpers::deserialize_one_or_many,
  traits::{ActivityHandler, Object}
};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{
  AppData,
  entities::{
    post::Model as DbPost,
    user::Model as DbUser,
  },
  error::Error,
  objects::post::Note,
};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreatePost {
  pub(crate) actor: ObjectId<DbUser>,
  #[serde(deserialize_with = "deserialize_one_or_many")]
  pub(crate) to: Vec<Url>,
  pub(crate) object: Note,
  #[serde(rename = "type")]
  pub(crate) kind: CreateType,
  pub(crate) id: Url,
}

impl CreatePost {
  pub async fn send(_note: Note, _inbox: Url, _data: &Data<AppData>) -> Result<(), Error> {
    todo!()
  }
}

#[async_trait::async_trait]
impl ActivityHandler for CreatePost {
  type DataType = AppData;
  type Error = Error;

  fn id(&self) -> &Url {
    &self.id
  }

  fn actor(&self) -> &Url {
    self.actor.inner()
  }

  async fn verify(&self, data: &Data<Self::DataType>) -> Result<(), Self::Error> {
    DbPost::verify(&self.object, &self.id, data).await?;
    Ok(())
  }

  async fn receive(self, data: &Data<Self::DataType>) -> Result<(), Self::Error> {
    DbPost::from_json(self.object, data).await?;
    Ok(())
  }
}