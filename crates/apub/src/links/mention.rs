use activitypub_federation::kinds::link::MentionType;
use serde::{Deserialize, Serialize};
use url::Url;
use utoipa::ToSchema;

/// <https://www.w3.org/ns/activitystreams#Mention>
#[derive(Clone, Debug, Deserialize, Serialize, ToSchema, Eq, PartialEq)]
pub struct Mention {
    #[schema(value_type = String)]
    #[serde(rename = "type")]
    pub kind: MentionType,
    pub href: Url,
    pub name: String,
}
