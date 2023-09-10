// https://github.com/LemmyNet/activitypub-federation-rust/blob/61085a643f05dbb70502b3c519fd666214b7e308/examples/live_federation/objects/post.rs
// https://github.com/LemmyNet/lemmy/blob/main/crates/apub/assets

use activitypub_federation::{
    config::Data,
    fetch::object_id::ObjectId,
    kinds::{public, object::NoteType},
    protocol::helpers::deserialize_one_or_many,
    traits::Actor,
};
use chrono::{Local, SecondsFormat};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{
    protocol::links::Mention,
    entities::{
        post::Model as DbPost,
        user::Model as DbUser,
    }, AppData, error::AppError
};

/// https://www.w3.org/ns/activitystreams#Note
#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Note {
    #[serde(rename = "type")]
    pub(crate) kind: NoteType,
    pub(crate) id: ObjectId<DbPost>,
    pub(crate) attributed_to: ObjectId<DbUser>,
    #[serde(deserialize_with = "deserialize_one_or_many")]
    pub(crate) to: Vec<Url>,
    #[serde(deserialize_with = "deserialize_one_or_many")]
    pub(crate) cc: Vec<Url>,
    pub(crate) content: String,
    /// TODO: customization via item._hatsu.source
    pub(crate) source: String,
    /// TODO: remove in_reply_to (version 0.1.0)
    pub(crate) in_reply_to: Option<ObjectId<DbPost>>,
    pub(crate) tag: Vec<Mention>,
    pub(crate) published: Option<String>,
    pub(crate) updated: Option<String>,
    // TODO:
    // sensitive (default: false) (extension: _hatsu.sensitive)
    // attachment
    // context (?)
    // conversation (?)
}

impl Note {
    pub fn new(note_id: String, actor: &DbUser, source: String, data: &Data<AppData>) -> Result<Self, AppError> {
        Ok(Self {
            kind: Default::default(),
            id: Url::parse(&note_id)?.into(),
            attributed_to: actor.id().into(),
            to: vec![public()],
            cc: vec![Url::parse(&format!("https://{}/u/{}/followers", data.domain(), actor.id()))?],
            content: markdown::to_html_with_options(&source, &markdown::Options::gfm()).unwrap(),
            source,
            in_reply_to: None,
            tag: vec![],
            published: Some(Local::now().to_rfc3339_opts(SecondsFormat::Secs, true)),
            updated: None,
        })
    }
}
