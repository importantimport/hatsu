use activitypub_federation::config::Data;
use hatsu_apub::{
    // actors::ApubUser, 
    // objects::{ApubPost, Note}
    objects::Note,
};
// use hatsu_db_schema::post::Model as DbPost;
use hatsu_utils::{AppData, AppError};
use serde::{Deserialize, Serialize};
// use std::ops::Deref;
use utoipa::ToSchema;

use crate::entities::{Account, CustomEmoji};

/// https://docs.joinmastodon.org/entities/Status/
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct Status {
    pub id: String,
    pub in_reply_to_id: Option<String>,
    pub uri: String,
    pub url: String,
    pub account: Account,
    pub created_at: String,
    pub content: String,
    /// until I figure it out, it should always be an empty vec
    pub emojis: Vec<CustomEmoji>,
    /// depends on context
    pub replies_count: u64,
    /// should always be 0
    pub reblogs_count: u64,
    /// should always be 0
    pub favourites_count: u64,
}

impl Status {
    pub async fn from_json(
        note: Note,
        _data: &Data<AppData>
    ) -> Result<Self, AppError> {
        // let user = note.attributed_to.dereference_local(data).await?;

        Ok(Self {
            id: note.id.to_string(),
            in_reply_to_id: note.in_reply_to.and_then(|url| Some(url.to_string())),
            // TODO: replace
            uri: note.id.to_string(),
            // TODO: replace
            url: note.id.to_string(),
            // TODO: Account::from_json()
            account: Account {  },
            // TODO: remove unwrap
            created_at: note.published.unwrap(),
            content: note.content,
            emojis: vec![],
            replies_count: 0,
            reblogs_count: 0,
            favourites_count: 0,
        })
    }
}
