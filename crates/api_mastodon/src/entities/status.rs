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
use url::Url;
use utoipa::ToSchema;

use crate::entities::{Account, CustomEmoji};

/// https://docs.joinmastodon.org/entities/Status/
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct Status {
    pub id: Url,
    pub in_reply_to_id: Option<Url>,
    pub uri: Url,
    pub url: Url,
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
            id: note.id.clone().into(),
            in_reply_to_id: note.in_reply_to.and_then(|in_reply_to| Some(in_reply_to.into())),
            // TODO: replace
            uri: note.id.clone().into(),
            // TODO: replace
            url: note.id.clone().into(),
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
