use std::ops::Deref;

use hatsu_db_schema::user_feed_item::UserFeedItemHatsu as DbUserFeedItemHatsu;
use serde::{Deserialize, Serialize};
use url::Url;

/// Hatsu JSON Feed Item Extension
///
/// <https://hatsu.cli.rs/others/json-feed-extension.html#items>
///
/// <https://github.com/importantimport/hatsu/issues/1>
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct UserFeedItemHatsu {
    pub about: Option<Url>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WrappedUserFeedItemHatsu(pub(crate) DbUserFeedItemHatsu);

impl AsRef<DbUserFeedItemHatsu> for WrappedUserFeedItemHatsu {
    fn as_ref(&self) -> &DbUserFeedItemHatsu {
        &self.0
    }
}

impl Deref for WrappedUserFeedItemHatsu {
    type Target = DbUserFeedItemHatsu;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<DbUserFeedItemHatsu> for WrappedUserFeedItemHatsu {
    fn from(u: DbUserFeedItemHatsu) -> Self {
        Self(u)
    }
}

impl UserFeedItemHatsu {
    #[must_use]
    pub fn into_db(self) -> DbUserFeedItemHatsu {
        DbUserFeedItemHatsu {
            about: self.about.map(|url| url.to_string()),
        }
    }
}
