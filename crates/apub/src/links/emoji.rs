use activitypub_federation::kinds::{kind, object::ImageType};
use serde::{Deserialize, Serialize};
use url::Url;
use utoipa::ToSchema;

kind!(EmojiType, Emoji);

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Emoji {
    #[schema(value_type = String)]
    #[serde(rename = "type")]
    pub kind: EmojiType,
    pub icon: EmojiIcon,
    pub id: Url,
    pub name: String,
    pub updated: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EmojiIcon {
    #[schema(value_type = String)]
    #[serde(rename = "type")]
    pub kind: ImageType,
    pub media_type: Option<String>,
    pub url: Url,
}
