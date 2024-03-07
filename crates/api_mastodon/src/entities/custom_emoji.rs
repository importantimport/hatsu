use hatsu_apub::actors::{ServiceTag, ServiceTagEmoji};
use serde::{Deserialize, Serialize};
use url::Url;
use utoipa::ToSchema;

/// <https://docs.joinmastodon.org/entities/CustomEmoji/>
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct CustomEmoji {
    shortcode: String,
    url: Url,
    static_url: Url,
    visible_in_picker: bool,
    category: Option<String>,
}

impl CustomEmoji {
    pub fn from_json(tag: Option<ServiceTag>) -> Option<Vec<Self>> {
        match tag {
            Some(ServiceTag::Emoji(emoji)) => Some(vec![Self::from_emoji(emoji)]),
            _ => None,
        }
    }

    pub fn from_emoji(emoji: ServiceTagEmoji) -> Self {
        Self {
            shortcode: emoji.name,
            url: emoji.icon.url.clone(),
            static_url: emoji.icon.url,
            visible_in_picker: false,
            category: None,
        }
    }
}
