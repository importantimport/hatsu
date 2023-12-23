// use activitypub_federation::traits::ActivityHandler;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
// #[enum_delegate::implement(ActivityHandler)]
pub enum SharedInboxActivities {
    // CreateOrUpdateNote(CreateOrUpdateNote),
    // Follow(Follow),
    // AcceptFollow(AcceptFollow),
    // UndoFollow(UndoFollow),
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
// #[enum_delegate::implement(ActivityHandler)]
pub enum ServiceInboxActivities {
    // CreateOrUpdateNote(CreateOrUpdateNote),
    // Follow(Follow),
    // AcceptFollow(AcceptFollow),
    // UndoFollow(UndoFollow),
}
