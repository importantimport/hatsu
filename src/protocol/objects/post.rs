// https://github.com/LemmyNet/activitypub-federation-rust/blob/61085a643f05dbb70502b3c519fd666214b7e308/examples/live_federation/objects/post.rs
// https://github.com/LemmyNet/lemmy/blob/main/crates/apub/assets

use activitypub_federation::{
    fetch::object_id::ObjectId,
    kinds::{
        link::MentionType,
        object::NoteType,
    },
    protocol::helpers::deserialize_one_or_many,
};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::entities::{
    post::Model as DbPost,
    user::Model as DbUser,
};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Note {
    #[serde(rename = "type")]
    pub(crate) kind: NoteType,
    pub(crate) id: ObjectId<DbPost>,
    pub(crate) attributed_to: ObjectId<DbUser>,
    #[serde(deserialize_with = "deserialize_one_or_many")]
    pub(crate) to: Vec<Url>,
    pub(crate) content: String,
    pub(crate) in_reply_to: Option<ObjectId<DbPost>>,
    pub(crate) tag: Vec<Mention>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Mention {
    pub href: Url,
    #[serde(rename = "type")]
    pub kind: MentionType,
}
