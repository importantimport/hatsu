use hatsu_db_schema::user::UserHatsu as DbUserHatsu;
use serde::{Deserialize, Serialize};
use url::Url;

/// Hatsu JSON Feed Extension
///
/// <https://hatsu.cli.rs/others/json-feed-extension.html#top-level>
///
/// <https://github.com/importantimport/hatsu/issues/1>
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct UserFeedHatsu {
    pub about: Option<Url>,
    pub aliases: Option<String>,
    pub banner_image: Option<Url>,
}

impl UserFeedHatsu {
    #[must_use]
    pub fn into_db(self) -> DbUserHatsu {
        DbUserHatsu {
            about: self.about.map(|url| url.to_string()),
            aliases: self.aliases,
            banner_image: self.banner_image.map(|url| url.to_string()),
        }
    }

    #[must_use]
    pub fn from_db(db_hatsu: DbUserHatsu) -> Self {
        Self {
            about: db_hatsu.about.and_then(|url| Url::parse(&url).ok()),
            aliases: db_hatsu.aliases,
            banner_image: db_hatsu.banner_image.and_then(|url| Url::parse(&url).ok()),
        }
    }
}
