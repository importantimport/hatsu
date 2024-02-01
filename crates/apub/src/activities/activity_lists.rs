use activitypub_federation::{config::Data, traits::ActivityHandler};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::activities::{
    AcceptFollow,
    CreateNote,
    // CreateOrUpdateNote,
    Follow,
    UndoFollow,
};

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
#[enum_delegate::implement(ActivityHandler)]
pub enum SharedInboxActivities {
    CreateNote(CreateNote),
    // CreateOrUpdateNote(CreateOrUpdateNote),
    Follow(Follow),
    AcceptFollow(AcceptFollow),
    UndoFollow(UndoFollow),
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
#[enum_delegate::implement(ActivityHandler)]
pub enum ServiceInboxActivities {
    CreateNote(CreateNote),
    // CreateOrUpdateNote(CreateOrUpdateNote),
    Follow(Follow),
    AcceptFollow(AcceptFollow),
    UndoFollow(UndoFollow),
}
