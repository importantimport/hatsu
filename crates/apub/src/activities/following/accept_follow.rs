use activitypub_federation::{
    config::Data,
    fetch::object_id::ObjectId,
    kinds::activity::AcceptType,
    protocol::{context::WithContext, helpers::deserialize_skip_error},
    traits::ActivityHandler,
};
use hatsu_db_schema::activity::Model as DbActivity;
use hatsu_utils::{AppData, AppError};
use sea_orm::{ActiveModelTrait, IntoActiveModel};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{activities::Follow, actors::ApubUser};

/// <https://github.com/LemmyNet/lemmy/blob/963d04b3526f8a5e9ff762960bfb5215e353bb27/crates/apub/src/protocol/activities/following/accept.rs>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AcceptFollow {
    pub(crate) actor: ObjectId<ApubUser>,
    /// Optional, for compatibility with platforms that always expect recipient field
    #[serde(deserialize_with = "deserialize_skip_error", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) to: Option<[ObjectId<ApubUser>; 1]>,
    pub(crate) object: Follow,
    #[serde(rename = "type")]
    pub(crate) kind: AcceptType,
    pub(crate) id: Url,
}

/// <https://github.com/LemmyNet/lemmy/blob/963d04b3526f8a5e9ff762960bfb5215e353bb27/crates/apub/src/activities/following/accept.rs>
/// <https://github.com/LemmyNet/activitypub-federation-rust/blob/7bb17f21d59b0aed6126d8a8a0cd60897cb02e6d/examples/local_federation/activities/accept.rs>
impl AcceptFollow {
    pub async fn new(follow: Follow, data: &Data<AppData>) -> Result<WithContext<Self>, AppError> {
        // 被关注者（本地账号），https://{}/users/{}
        let user: ApubUser = follow.object.dereference_local(data).await?;
        // 关注者
        let person = follow.actor.clone().dereference(data).await?;
        // 接受关注
        let activity = Self {
            actor: Url::parse(&user.id)?.into(),
            to: Some([Url::parse(&person.id)?.into()]),
            object: follow,
            kind: AcceptType::Accept,
            // 使用 UUID v7 作为 ID
            id: hatsu_utils::url::generate_activity_url(data.domain(), None)?,
        };

        let _insert_activity = DbActivity {
            id: activity.id().to_string(),
            activity: serde_json::to_string(&activity)?,
            actor: activity.actor().to_string(),
            kind: activity.kind.to_string(),
            published: Some(hatsu_utils::date::now()),
        }
        .into_active_model()
        .insert(&data.conn)
        .await?;

        Ok(WithContext::new_default(activity))
    }
}

/// 只发送，不接收
/// send only, without receive
/// <https://github.com/LemmyNet/lemmy/blob/963d04b3526f8a5e9ff762960bfb5215e353bb27/crates/apub/src/activities/following/accept.rs>
#[async_trait::async_trait]
impl ActivityHandler for AcceptFollow {
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
