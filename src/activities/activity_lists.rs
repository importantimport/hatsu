use activitypub_federation::{
  config::Data,
  traits::ActivityHandler
};
use serde::{Deserialize, Serialize};
use url::Url;

use super::{
  create_post::CreatePost,
  following::{
    follow::Follow,
    accept::AcceptFollow,
    undo_follow::UndoFollow,
  }
};

/// 用户 Inbox 接受的 Activity 类型
#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
#[enum_delegate::implement(ActivityHandler)]
pub enum PersonInboxActivities {
    CreateNote(CreatePost),
    Follow(Follow),
    AcceptFollow(AcceptFollow),
    UndoFollow(UndoFollow),
}
