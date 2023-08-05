use activitypub_federation::{
    config::Data,
    protocol::verification::verify_domains_match,
    traits::Object,
};
use sea_orm::*;
use url::Url;

use crate::{
    AppData,
    AppError,
    protocol::activities::activity_lists::PersonInboxActivities,
    entities::{
        prelude::*,
        activity::Model as DbActivity,
    },
};

#[async_trait::async_trait]
impl Object for DbActivity {
    type DataType = AppData;
    /// 所有支持的 Activity 类型
    /// https://github.com/LemmyNet/activitypub-federation-rust/blob/main/docs/10_fetching_objects_with_unknown_type.md
    type Kind = PersonInboxActivities;
    type Error = AppError;

    async fn read_from_id(
        activity_id: Url,
        data: &Data<Self::DataType>,
    ) -> Result<Option<Self>, Self::Error> {
        Ok(Activity::find_by_id(&activity_id.to_string())
            .one(&data.conn)
            .await?)
    }

    async fn into_json(self, _data: &Data<Self::DataType>) -> Result<Self::Kind, Self::Error> {
        todo!()
    }

    async fn verify(
        json: &Self::Kind,
        expected_domain: &url::Url,
        _data: &Data<Self::DataType>,
    ) -> Result<(), Self::Error> {
        use PersonInboxActivities::*;
        
        match json {
            AcceptFollow(json) => verify_domains_match(json.id.inner(), expected_domain)?,
            Follow(json) => verify_domains_match(json.id.inner(), expected_domain)?,
            UndoFollow(json) => verify_domains_match(json.id.inner(), expected_domain)?,
            CreateOrUpdateNote(json) => verify_domains_match(json.id.inner(), expected_domain)?,
        }

        Ok(())    
    }

    async fn from_json(_json: Self::Kind, _data: &Data<Self::DataType>) -> Result<Self, Self::Error> {
        // use PersonInboxActivities::*;
        // match json {
        //     PersonInboxActivities::AcceptFollow() => Ok(()),
        //     PersonInboxActivities::Follow() => Ok(()),
        //     PersonInboxActivities::UndoFollow() => Ok(()),
        //     PersonInboxActivities::CreateOrUpdateNote() => Ok(())
        // }
        todo!()
    }
}