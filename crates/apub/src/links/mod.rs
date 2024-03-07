use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::ToSchema;

mod emoji;
mod hashtag;
mod mention;

pub use emoji::{Emoji, EmojiIcon};
pub use hashtag::Hashtag;
pub use mention::Mention;

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
#[serde(untagged)]
pub enum Tag {
    Emoji(Emoji),
    Hashtag(Hashtag),
    Mention(Mention),
    Any(Value),
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
#[serde(untagged)]
pub enum Tags {
    Tag(Tag),
    Tags(Vec<Tag>),
}
