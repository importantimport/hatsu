use activitypub_federation::kinds::kind;
use serde::{Deserialize, Serialize};
use url::Url;

kind!(HashtagType, Hashtag);

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Hashtag {
    #[serde(rename = "type")]
    pub kind: HashtagType,
    /// <https://hatsu.local/t/foo>
    pub href: Url,
    /// #foo
    pub name: String,
}

impl Hashtag {
    pub fn new(href: Url, name: String) -> Self {
        Self {
            kind: HashtagType::Hashtag,
            href,
            name,
        }
    }
}
