use activitypub_federation::kinds::activity::FollowType;
use hatsu_db_schema::received_follow::Model as DbReceivedFollow;
use hatsu_utils::AppError;
use url::Url;

use crate::activities::{ApubReceivedFollow, Follow};

impl ApubReceivedFollow {
    // 转换为ActivityStreams JSON
    pub fn into_json(self) -> Result<Follow, AppError> {
        Ok(Follow {
            kind: FollowType::Follow,
            id: Url::parse(&self.id)?,
            actor: Url::parse(&self.actor)?.into(),
            to: self.to.clone().map(|to| serde_json::from_str(&to).unwrap()),
            object: Url::parse(&self.object)?.into(),
        })
    }

    // 转换为本地格式
    pub fn from_json(json: Follow) -> Result<Self, AppError> {
        let received_follow = DbReceivedFollow {
            id: json.id.to_string(),
            actor: json.actor.to_string(),
            to: json.to.map(|to| serde_json::to_string(&to).unwrap()),
            object: json.object.to_string(),
        };

        Ok(received_follow.into())
    }
}
