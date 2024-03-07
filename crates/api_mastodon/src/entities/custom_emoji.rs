use hatsu_apub::links::{Emoji, Tag, Tags};
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
    pub fn from_json(tag: Option<Tags>) -> Vec<Self> {
        let emojis = match tag {
            Some(Tags::Tag(tag)) => vec![Self::from_tag(tag)],
            Some(Tags::Tags(tags)) => tags.into_iter().map(|tag| Self::from_tag(tag)).collect(),
            _ => vec![],
        };

        emojis.into_iter().filter_map(|emoji| emoji).collect()
    }

    pub fn from_tag(tag: Tag) -> Option<Self> {
        match tag {
            Tag::Emoji(emoji) => Some(Self::from_emoji(emoji)),
            _ => None,
        }
    }

    pub fn from_emoji(emoji: Emoji) -> Self {
        Self {
            shortcode: emoji.name,
            url: emoji.icon.url.clone(),
            static_url: emoji.icon.url,
            visible_in_picker: false,
            category: None,
        }
    }
}
