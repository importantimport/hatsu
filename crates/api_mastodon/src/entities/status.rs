use activitypub_federation::{
    config::Data,
    traits::Object,
};
use hatsu_apub::objects::Note;
use hatsu_utils::{AppData, AppError};
use serde::{Deserialize, Serialize};
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
        data: &Data<AppData>
    ) -> Result<Self, AppError> {
        let apub_user = note.attributed_to.dereference_local(data).await?;
        let service = apub_user.into_json(data).await?;

        Ok(Self {
            id: note.id.clone().into(),
            in_reply_to_id: note.in_reply_to.map(|in_reply_to| in_reply_to.into()),
            // TODO: replace
            uri: note.id.clone().into(),
            // TODO: replace
            url: note.id.clone().into(),
            account: Account::from_json(service)?,
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
