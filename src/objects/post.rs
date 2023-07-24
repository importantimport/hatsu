// https://github.com/LemmyNet/activitypub-federation-rust/blob/61085a643f05dbb70502b3c519fd666214b7e308/examples/live_federation/objects/post.rs
// https://github.com/LemmyNet/lemmy/blob/main/crates/apub/assets

use std::env;

use activitypub_federation::{
    config::Data,
    fetch::object_id::ObjectId,
    kinds::{object::NoteType, public},
    protocol::{helpers::deserialize_one_or_many, verification::verify_domains_match},
    traits::{Actor, Object},
};
use activitystreams_kinds::link::MentionType;
use sea_orm::*;
use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

use crate::{
    AppData,
    activities::create_post::CreatePost,
    entities::{
        prelude::*,
        post::Model as DbPost,
        user::Model as DbUser,
    },
    error::Error,
};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Note {
    #[serde(rename = "type")]
    kind: NoteType,
    id: ObjectId<DbPost>,
    pub(crate) attributed_to: ObjectId<DbUser>,
    #[serde(deserialize_with = "deserialize_one_or_many")]
    pub(crate) to: Vec<Url>,
    content: String,
    in_reply_to: Option<ObjectId<DbPost>>,
    tag: Vec<Mention>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Mention {
    pub href: Url,
    #[serde(rename = "type")]
    pub kind: MentionType,
}

#[async_trait::async_trait]
impl Object for DbPost {
    type DataType = AppData;
    type Kind = Note;
    type Error = Error;

    async fn read_from_id(
        object_id: Url,
        data: &Data<Self::DataType>
    ) -> Result<Option<Self>, Self::Error> {
        let db_post: Option<DbPost> = Post::find_by_id(&object_id.to_string())
            .one(&data.conn)
            .await?;

        Ok(db_post)
    }

    async fn into_json(self, data: &Data<Self::DataType>) -> Result<Self::Kind, Self::Error> {
        // TODO: 不确定是否可用
        let object_id: ObjectId<DbUser> = Url::parse(&self.creator)?.into();
        let creator = object_id.dereference_local(data).await?;
        let mention = Mention {
            href: Url::parse(&creator.id).unwrap(),
            kind: Default::default()
        };
        let note = Note {
            kind: Default::default(),
            id: Url::parse(&self.id)?.into(),
            attributed_to: Url::parse(&self.creator)?.into(),
            // TODO:
            // to: vec![public(), creator.followers_url()?],
            to: vec![public()],
            content: self.text,
            in_reply_to: None,
            tag: vec![mention]
        };

        Ok(note)
    }

    async fn verify(
        json: &Self::Kind,
        expected_domain: &Url,
        _data: &Data<Self::DataType>
    ) -> Result<(), Self::Error> {
        verify_domains_match(json.id.inner(), expected_domain)?;
        Ok(())
    }

    async fn from_json(json: Self::Kind, data: &Data<Self::DataType>) -> Result<Self, Self::Error> {
        tracing::info!("Received post with content {} and id {}", &json.content, &json.id);

        let creator = json.attributed_to.dereference(data).await?;
        let post = DbPost {
            id: json.id.to_string(),
            creator: json.attributed_to.to_string(),
            text: json.content,
            local: false,
        };

        let mention = Mention {
            href: Url::parse(&creator.id).unwrap(),
            kind: Default::default()
        };
        let note = Note {
            kind: Default::default(),
            id: Url::parse(&format!("https://{}/o/{}", data.domain(), Uuid::new_v4()))?.into(),
            // TODO: multiple user
            attributed_to: Url::parse(&format!("https://{}/u/{}", data.domain(), env::var("HATSU_TEST_ACCOUNT").unwrap()))?.into(),
            to: vec![public()],
            content: format!("Hello {}", creator.name),
            in_reply_to: Some(json.id.clone()),
            tag: vec![mention]
        };

        CreatePost::send(note, creator.shared_inbox_or_inbox(), data).await?;

        Ok(post)
    }

}