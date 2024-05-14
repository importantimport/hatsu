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
    pub banner_image: Option<Url>,
}
