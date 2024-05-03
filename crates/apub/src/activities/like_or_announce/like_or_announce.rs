use std::ops::Deref;

use activitypub_federation::{config::Data, fetch::object_id::ObjectId, traits::ActivityHandler};
use hatsu_db_schema::{
    prelude::{ReceivedAnnounce, ReceivedLike},
    received_announce,
    received_like,
};
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

use crate::{
    activities::{ApubReceivedAnnounce, ApubReceivedLike, LikeOrAnnounceType},
    actors::ApubUser,
    objects::ApubPost,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LikeOrAnnounce {
    #[serde(rename = "type")]
    pub(crate) kind: LikeOrAnnounceType,
    pub(crate) id: Url,
    pub(crate) actor: ObjectId<ApubUser>,
    pub(crate) object: ObjectId<ApubPost>,
}

/// receive only
#[async_trait::async_trait]
impl ActivityHandler for LikeOrAnnounce {
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

        match self.kind {
            LikeOrAnnounceType::LikeType(_) => {
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
                    ApubReceivedLike::from_json(&self)?
                        .deref()
                        .clone()
                        .into_active_model()
                        .insert(&data.conn)
                        .await?;
                }
            },
            LikeOrAnnounceType::AnnounceType(_) => {
                if ReceivedAnnounce::find()
                    .filter(
                        Condition::all()
                            .add(received_announce::Column::Actor.eq(&actor.id))
                            .add(received_announce::Column::Object.eq(&object.id)),
                    )
                    .one(&data.conn)
                    .await?
                    .is_none()
                {
                    ApubReceivedAnnounce::from_json(&self)?
                        .deref()
                        .clone()
                        .into_active_model()
                        .insert(&data.conn)
                        .await?;
                }
            },
        }

        Ok(())
    }
}
