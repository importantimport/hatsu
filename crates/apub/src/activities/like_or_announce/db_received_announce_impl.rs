use activitypub_federation::kinds::activity::AnnounceType;
use hatsu_db_schema::received_announce::Model as DbReceivedAnnounce;
use hatsu_utils::AppError;
use url::Url;

use crate::activities::{ApubReceivedAnnounce, LikeOrAnnounce, LikeOrAnnounceType};

impl ApubReceivedAnnounce {
    pub fn into_json(self) -> Result<LikeOrAnnounce, AppError> {
        Ok(LikeOrAnnounce {
            kind: LikeOrAnnounceType::AnnounceType(AnnounceType::Announce),
            id: Url::parse(&self.id)?,
            actor: Url::parse(&self.actor)?.into(),
            object: Url::parse(&self.object)?.into(),
        })
    }

    pub fn from_json(json: &LikeOrAnnounce) -> Result<Self, AppError> {
        Ok(DbReceivedAnnounce {
            id: json.id.to_string(),
            actor: json.actor.to_string(),
            object: json.object.to_string(),
        }
        .into())
    }
}
