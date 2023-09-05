use activitypub_federation::{
    config::Data,
    fetch::object_id::ObjectId,
    protocol::{
        helpers::deserialize_one_or_many,
        context::WithContext
    },
    traits::{ActivityHandler, Object},
};
use sea_orm::*;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{
    AppData,
    AppError,
    entities::{
        activity::Model as DbActivity,
        user::Model as DbUser,
        post::Model as DbPost,
    },
    protocol::{
        activities::CreateOrUpdateType,
        objects::Note
    },
    utilities::generate_activity_url
};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateOrUpdateNote {
    pub(crate) actor: ObjectId<DbUser>,
    #[serde(deserialize_with = "deserialize_one_or_many")]
    pub(crate) to: Vec<Url>,
    pub(crate) object: Note,
    #[serde(deserialize_with = "deserialize_one_or_many")]
    pub(crate) cc: Vec<Url>,
    #[serde(rename = "type")]
    pub(crate) kind: CreateOrUpdateType,
    pub(crate) id: Url,
}

impl CreateOrUpdateNote {
    pub async fn new(
        note: Note,
        kind: CreateOrUpdateType,
        data: &Data<AppData>
    ) -> Result<WithContext<Self>, AppError> {
        let activity = Self {
            id: generate_activity_url(data.domain(), None)?,
            actor: note.attributed_to.clone(),
            to: note.to.clone(),
            cc: note.cc.clone(),
            object: note.clone(),
            kind
        };

        let _insert_activity = DbActivity {
            id: activity.id().to_string(),
            activity: serde_json::to_string(&activity)?,
            actor: activity.actor().to_string(),
            kind: activity.kind.to_string(),
            published: note.published,
        }
            .into_active_model()
            .insert(&data.conn)
            .await?;

        Ok(WithContext::new_default(activity))
    }
}

#[async_trait::async_trait]
impl ActivityHandler for CreateOrUpdateNote {
    type DataType = AppData;
    type Error = AppError;

    fn id(&self) -> &Url {
        &self.id
    }

    fn actor(&self) -> &Url {
        self.actor.inner()
    }

    async fn verify(&self, data: &Data<Self::DataType>) -> Result<(), Self::Error> {
        // TODO
        DbPost::verify(&self.object, &self.id, data).await?;
        Ok(())
    }

    async fn receive(self, data: &Data<Self::DataType>) -> Result<(), Self::Error> {
        // TODO
        DbPost::from_json(self.object, data).await?;
        Ok(())
    }
}

