use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

// use crate::entities::CustomEmoji;

/// https://docs.joinmastodon.org/entities/Account/
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct Account {
    // pub id: String,
    // pub username: String,
    // pub url: String,
    // pub display_name: String,
    // pub avatar: String,
    // pub avatar_static: String,
    // pub emojis: Option<CustomEmoji>,
}
