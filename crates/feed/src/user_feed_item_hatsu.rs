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
