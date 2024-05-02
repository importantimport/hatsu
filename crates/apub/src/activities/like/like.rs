use std::ops::Deref;

use activitypub_federation::{
    config::Data,
    fetch::object_id::ObjectId,
    kinds::activity::LikeType,
    traits::ActivityHandler,
};
use hatsu_db_schema::{prelude::ReceivedLike, received_like};
use hatsu_utils::{AppData, AppError};
use sea_orm::{
    ActiveModelTrait,
    ColumnTrait,
    Condition,
    EntityTrait,
    IntoActiveModel,
    QueryFilter,
};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{activities::ApubReceivedLike, actors::ApubUser, objects::ApubPost};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Like {
    #[serde(rename = "type")]
    pub(crate) kind: LikeType,
    pub(crate) id: Url,
    pub(crate) actor: ObjectId<ApubUser>,
    pub(crate) object: ObjectId<ApubPost>,
}

/// receive only
#[async_trait::async_trait]
impl ActivityHandler for Like {
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
        let actor = self.actor.dereference(data).await?;
        let object = self.object.dereference_local(data).await?;

        if ReceivedLike::find()
            .filter(
                Condition::all()
                    .add(received_like::Column::Actor.eq(&actor.id))
                    .add(received_like::Column::Object.eq(&object.id)),
            )
            .one(&data.conn)
            .await?
            .is_none()
        {
            ApubReceivedLike::from_json(self)?
                .deref()
                .clone()
                .into_active_model()
                .insert(&data.conn)
                .await?;
        }

        Ok(())
    }
}
