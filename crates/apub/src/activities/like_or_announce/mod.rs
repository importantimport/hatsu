use std::fmt::{Display, Formatter, Result};

use activitypub_federation::kinds::activity::{AnnounceType, LikeType};
use serde::{Deserialize, Serialize};

mod db_received_announce;
mod db_received_announce_impl;
mod db_received_like;
mod db_received_like_impl;

mod like_or_announce;
mod undo_like_or_announce;

pub use db_received_announce::ApubReceivedAnnounce;
pub use db_received_like::ApubReceivedLike;
pub use like_or_announce::LikeOrAnnounce;
pub use undo_like_or_announce::UndoLikeOrAnnounce;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum LikeOrAnnounceType {
    LikeType(LikeType),
    AnnounceType(AnnounceType),
}

impl Display for LikeOrAnnounceType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self::LikeType(_) => f.write_str(&LikeType::Like.to_string()),
            Self::AnnounceType(_) => f.write_str(&AnnounceType::Announce.to_string()),
        }
    }
}
