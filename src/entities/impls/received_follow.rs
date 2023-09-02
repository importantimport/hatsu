use url::Url;

use crate::{
    AppError,
    entities::received_follow::Model as DbReceivedFollow,
    protocol::activities::Follow,
};

impl DbReceivedFollow {
    // 转换为ActivityStreams JSON
    pub async fn into_json(self) -> Result<Follow, AppError> {
        Ok(Follow {
            kind: Default::default(),
            id: Url::parse(&self.id)?,
            actor: Url::parse(&self.actor)?.into(),
            to: self.to.and_then(|to| serde_json::from_str(&to).unwrap()),
            object: Url::parse(&self.object)?.into(),
        })
    }

    // 转换为本地格式
    pub async fn from_json(json: Follow) -> Result<Self, AppError> {
        Ok(Self {
            id: json.id.to_string(),
            actor: json.actor.to_string(),
            to: json.to.and_then(|to| Some(serde_json::to_string(&to).unwrap())),
            object: json.object.to_string()
        })
    }
}