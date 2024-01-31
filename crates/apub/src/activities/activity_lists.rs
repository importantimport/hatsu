use activitypub_federation::{config::Data, traits::ActivityHandler};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::activities::{AcceptFollow, CreateNote, Follow, UndoFollow};

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
#[enum_delegate::implement(ActivityHandler)]
pub enum SharedInboxActivities {
    // CreateOrUpdateNote(CreateOrUpdateNote),
    CreateNote(CreateNote),
    Follow(Follow),
    AcceptFollow(AcceptFollow),
    UndoFollow(UndoFollow),
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
#[enum_delegate::implement(ActivityHandler)]
pub enum ServiceInboxActivities {
    // CreateOrUpdateNote(CreateOrUpdateNote),
    CreateNote(CreateNote),
    Follow(Follow),
    AcceptFollow(AcceptFollow),
    UndoFollow(UndoFollow),
}
