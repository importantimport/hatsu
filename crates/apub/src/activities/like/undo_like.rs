use activitypub_federation::{
    config::Data,
    fetch::object_id::ObjectId,
    kinds::activity::UndoType,
    traits::ActivityHandler,
};
use hatsu_db_schema::prelude::ReceivedLike;
use hatsu_utils::{AppData, AppError};
use sea_orm::EntityTrait;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{activities::Like, actors::ApubUser};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UndoLike {
    #[serde(rename = "type")]
    pub(crate) kind: UndoType,
    pub(crate) id: Url,
    pub(crate) actor: ObjectId<ApubUser>,
    pub(crate) object: Like,
}

/// receive only
#[async_trait::async_trait]
impl ActivityHandler for UndoLike {
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
        ReceivedLike::delete_by_id(self.object.id)
            .exec(&data.conn)
            .await?;

        Ok(())
    }
}
