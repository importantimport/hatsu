use activitypub_federation::kinds::kind;
use serde::{Deserialize, Serialize};
use url::Url;
use utoipa::ToSchema;

kind!(HashtagType, Hashtag);

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema, Eq, PartialEq)]
pub struct Hashtag {
    #[schema(value_type = String)]
    #[serde(rename = "type")]
    pub kind: HashtagType,
    /// <https://hatsu.local/t/foo>
    pub href: Url,
    /// #foo
    pub name: String,
}

impl Hashtag {
    #[must_use]
    pub const fn new(href: Url, name: String) -> Self {
        Self {
            kind: HashtagType::Hashtag,
            href,
            name,
        }
    }
}
