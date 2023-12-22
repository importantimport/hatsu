use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::entities::{Account, CustomEmoji};

/// https://docs.joinmastodon.org/entities/Status/
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct Status {
    pub id: String,
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
