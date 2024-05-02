use std::collections::HashMap;

use serde::Serialize;
use url::Url;
use utoipa::ToSchema;

/// impl `ToSchema` for `Webfinger`
#[derive(Serialize, ToSchema)]
pub struct WebfingerSchema {
    /// The actor which is described here, for example `acct:LemmyDev@mastodon.social`
    pub subject: String,
    /// Links where further data about `subject` can be retrieved
    pub links: Vec<WebfingerSchemaLink>,
    /// Other Urls which identify the same actor as the `subject`
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub aliases: Vec<Url>,
    /// Additional data about the subject
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub properties: HashMap<Url, String>,
}

/// impl `ToSchema` for `WebfingerLink`
#[derive(Serialize, ToSchema)]
pub struct WebfingerSchemaLink {
    /// Relationship of the link, such as `self` or `http://webfinger.net/rel/profile-page`
    pub rel: Option<String>,
    /// Media type of the target resource
    #[serde(rename = "type")]
    pub kind: Option<String>,
    /// Url pointing to the target resource
    pub href: Option<Url>,
    /// Used for remote follow external interaction url
    pub template: Option<String>,
    /// Additional data about the link
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub properties: HashMap<Url, String>,
}
