use activitypub_federation::kinds::activity::LikeType;
use hatsu_db_schema::received_like::Model as DbReceivedLike;
use hatsu_utils::AppError;
use url::Url;

use crate::activities::{ApubReceivedLike, Like};

impl ApubReceivedLike {
    pub fn into_json(self) -> Result<Like, AppError> {
        Ok(Like {
            kind: LikeType::Like,
            id: Url::parse(&self.id)?,
            actor: Url::parse(&self.actor)?.into(),
            object: Url::parse(&self.object)?.into(),
        })
    }

    pub fn from_json(json: Like) -> Result<Self, AppError> {
        Ok(DbReceivedLike {
            id: json.id.to_string(),
            actor: json.actor.to_string(),
            object: json.object.to_string(),
        }
        .into())
    }
}
