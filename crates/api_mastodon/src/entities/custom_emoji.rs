use hatsu_apub::links::{Emoji, Tag};
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
    #[serde(skip_serializing_if = "Option::is_none")]
    category: Option<String>,
}

impl CustomEmoji {
    #[must_use]
    pub fn from_json(tags: Vec<Tag>) -> Vec<Self> {
        tags.into_iter()
            .filter_map(|tag| match tag {
                Tag::Emoji(emoji) => Some(Self::from_emoji(emoji)),
                _ => None,
            })
            .collect()
    }

    #[must_use]
    pub fn from_emoji(emoji: Emoji) -> Self {
        Self {
            shortcode: emoji
                .name
                .trim_start_matches(':')
                .trim_end_matches(':')
                .to_string(),
            url: emoji.icon.url.clone(),
            static_url: emoji.icon.url,
            visible_in_picker: false,
            category: None,
        }
    }
}
