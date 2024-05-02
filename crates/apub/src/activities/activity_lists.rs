use activitypub_federation::{config::Data, traits::ActivityHandler};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::activities::{AcceptFollow, CreateOrUpdateNote, Follow, Like, UndoFollow, UndoLike};

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
#[enum_delegate::implement(ActivityHandler)]
pub enum SharedInboxActivities {
    CreateOrUpdateNote(CreateOrUpdateNote),
    Follow(Follow),
    AcceptFollow(AcceptFollow),
    UndoFollow(UndoFollow),
    Like(Like),
    UndoLike(UndoLike),
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
#[enum_delegate::implement(ActivityHandler)]
pub enum ServiceInboxActivities {
    CreateOrUpdateNote(CreateOrUpdateNote),
    Follow(Follow),
    AcceptFollow(AcceptFollow),
    UndoFollow(UndoFollow),
    Like(Like),
    UndoLike(UndoLike),
}
