use activitypub_federation::{
    config::Data,
    fetch::object_id::ObjectId,
    kinds::activity::CreateType,
    protocol::{context::WithContext, helpers::deserialize_one_or_many},
    traits::{ActivityHandler, Object},
};
use chrono::{Local, SecondsFormat};
use hatsu_db_schema::activity::Model as DbActivity;
use hatsu_utils::{AppData, AppError};
use sea_orm::{ActiveModelTrait, IntoActiveModel};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{
    actors::ApubUser,
    objects::{ApubPost, Note},
};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateNote {
    pub actor: ObjectId<ApubUser>,
    #[serde(deserialize_with = "deserialize_one_or_many")]
    pub to: Vec<Url>,
    pub object: Note,
    #[serde(deserialize_with = "deserialize_one_or_many")]
    pub cc: Vec<Url>,
    #[serde(rename = "type")]
    pub kind: CreateType,
    pub id: Url,
    pub published: String,
}

impl CreateNote {
    pub async fn new(note: Note, data: &Data<AppData>) -> Result<WithContext<Self>, AppError> {
        let activity = Self {
            id: hatsu_utils::url::generate_activity_url(data.domain(), None)?,
            actor: note.attributed_to.clone(),
            to: note.to.clone(),
            cc: note.cc.clone(),
            object: note.clone(),
            kind: CreateType::Create,
            published: Local::now().to_rfc3339_opts(SecondsFormat::Secs, true),
        };

        let _insert_activity = DbActivity {
            id: activity.id().to_string(),
            activity: serde_json::to_string(&activity)?,
            actor: activity.actor().to_string(),
            kind: activity.kind.to_string(),
            published: Some(activity.published.clone()),
        }
        .into_active_model()
        .insert(&data.conn)
        .await?;

        Ok(WithContext::new_default(activity))
    }
}

#[async_trait::async_trait]
impl ActivityHandler for CreateNote {
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
        ApubPost::verify(&self.object, &self.id, data).await?;
        Ok(())
    }

    async fn receive(self, data: &Data<Self::DataType>) -> Result<(), Self::Error> {
        // TODO
        ApubPost::from_json(self.object, data).await?;
        Ok(())
    }
}
