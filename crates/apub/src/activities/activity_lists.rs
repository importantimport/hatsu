use activitypub_federation::{config::Data, traits::ActivityHandler};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::activities::{
    AcceptFollow,
    CreateOrUpdateNote,
    Follow,
    LikeOrAnnounce,
    UndoFollow,
    UndoLikeOrAnnounce,
};

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
#[enum_delegate::implement(ActivityHandler)]
pub enum UserInboxActivities {
    CreateOrUpdateNote(CreateOrUpdateNote),
    Follow(Follow),
    AcceptFollow(AcceptFollow),
    UndoFollow(UndoFollow),
    LikeOrAnnounce(LikeOrAnnounce),
    UndoLikeOrAnnounce(UndoLikeOrAnnounce),
}
