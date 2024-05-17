use hatsu_db_schema::user::UserFeed as DbUserFeed;
// use hatsu_feed::UserFeed;
use serde::{Deserialize, Serialize};
use url::Url;
use utoipa::ToSchema;

/// Hatsu User Attachment (Metadata)
#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserAttachment {
    /// should be `PropertyValue`
    #[schema(value_type = String)]
    #[serde(rename = "type")]
    pub kind: String,
    /// Website / JSON Feed / Atom Feed / RSS Feed
    pub name: String,
    /// html string
    pub value: String,
}

impl UserAttachment {
    #[must_use]
    pub fn new(name: &str, value: String) -> Self {
        Self {
            kind: String::from("PropertyValue"),
            name: String::from(name),
            value,
        }
    }

    #[must_use]
    pub fn generate(domain: &Url, feed: DbUserFeed) -> Vec<Self> {
        let mut attachment = vec![
            Self::new("Website", format!("<a href=\"{domain}\" rel=\"nofollow noreferrer noopener me\" target=\"_blank\" translate=\"no\">{domain}</a>"))
        ];

        if let Some(json) = feed.json {
            attachment.push(Self::new("JSON Feed", format!("<a href=\"{json}\" rel=\"nofollow noreferrer noopener\" target=\"_blank\" translate=\"no\">{json}</a>")));
        };

        if let Some(atom) = feed.atom {
            attachment.push(Self::new("Atom Feed", format!("<a href=\"{atom}\" rel=\"nofollow noreferrer noopener\" target=\"_blank\" translate=\"no\">{atom}</a>")));
        };

        if let Some(rss) = feed.rss {
            attachment.push(Self::new("RSS Feed", format!("<a href=\"{rss}\" rel=\"nofollow noreferrer noopener\" target=\"_blank\" translate=\"no\">{rss}</a>")));
        };

        attachment
    }
}
