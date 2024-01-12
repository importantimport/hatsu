use activitypub_federation::kinds::link::MentionType;
use serde::{Deserialize, Serialize};
use url::Url;

/// https://www.w3.org/ns/activitystreams#Mention
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Mention {
    pub href: Url,
    #[serde(rename = "type")]
    pub kind: MentionType,
}
