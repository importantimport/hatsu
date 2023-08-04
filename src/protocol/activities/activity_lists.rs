use activitypub_federation::{
  config::Data,
  traits::ActivityHandler
};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::protocol::activities::{
    // create_post::CreatePost,
    CreateOrUpdateNote,
    AcceptFollow,
    Follow,
    UndoFollow
};

/// 用户 Inbox 接受的 Activity 类型
#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
#[enum_delegate::implement(ActivityHandler)]
pub enum PersonInboxActivities {
    CreateOrUpdateNote(CreateOrUpdateNote),
    // CreateNote(CreatePost),
    Follow(Follow),
    AcceptFollow(AcceptFollow),
    UndoFollow(UndoFollow),
}
