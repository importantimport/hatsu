use activitypub_federation::{
    config::Data,
    fetch::object_id::ObjectId,
    kinds::activity::{CreateType, UpdateType},
    protocol::{context::WithContext, helpers::deserialize_one_or_many},
    traits::{ActivityHandler, Object},
};
use hatsu_db_schema::activity::Model as DbActivity;
use hatsu_utils::{AppData, AppError};
use sea_orm::{ActiveModelTrait, IntoActiveModel};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{
    activities::CreateOrUpdateType,
    actors::ApubUser,
    objects::{ApubPost, Note},
};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateOrUpdateNote {
    pub id: Url,
    #[serde(rename = "type")]
    pub kind: CreateOrUpdateType,
    pub actor: ObjectId<ApubUser>,
    pub published: String,
    #[serde(deserialize_with = "deserialize_one_or_many")]
    pub to: Vec<Url>,
    #[serde(deserialize_with = "deserialize_one_or_many")]
    pub cc: Vec<Url>,
    pub object: Note,
}

impl CreateOrUpdateNote {
    pub async fn new(
        note: Note,
        kind: CreateOrUpdateType,
        data: &Data<AppData>,
    ) -> Result<WithContext<Self>, AppError> {
        let activity = Self {
            kind,
            id: hatsu_utils::url::generate_activity_url(data.domain(), None)?,
            published: note.published.clone(),
            actor: note.attributed_to.clone(),
            to: note.to.clone(),
            cc: note.cc.clone(),
            object: note.clone(),
        };

        let _insert_activity = DbActivity {
            id: activity.id().to_string(),
            kind: activity.kind.to_string(),
            published: Some(activity.published.clone()),
            actor: activity.actor().to_string(),
            activity: serde_json::to_string(&activity)?,
        }
        .into_active_model()
        .insert(&data.conn)
        .await?;

        Ok(WithContext::new_default(activity))
    }

    pub async fn create(note: Note, data: &Data<AppData>) -> Result<WithContext<Self>, AppError> {
        Self::new(
            note,
            CreateOrUpdateType::CreateType(CreateType::Create),
            data,
        )
        .await
    }

    pub async fn update(note: Note, data: &Data<AppData>) -> Result<WithContext<Self>, AppError> {
        Self::new(
            note,
            CreateOrUpdateType::UpdateType(UpdateType::Update),
            data,
        )
        .await
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
        ApubPost::verify(&self.object, &self.id, data).await?;
        Ok(())
    }

    async fn receive(self, data: &Data<Self::DataType>) -> Result<(), Self::Error> {
        match self.kind {
            CreateOrUpdateType::CreateType(_) => ApubPost::from_json(self.object, data).await?,
            CreateOrUpdateType::UpdateType(_) => todo!(), // TODO
        };

        Ok(())
    }
}
