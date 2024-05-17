use activitypub_federation::kinds::object::ImageType;
use serde::{Deserialize, Serialize};
use url::Url;
use utoipa::ToSchema;

/// Hatsu User Image
#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserImage {
    #[schema(value_type = String)]
    #[serde(rename = "type")]
    pub kind: ImageType,
    // image src
    pub url: Url,
}

impl UserImage {
    #[must_use]
    pub const fn new(url: Url) -> Self {
        Self {
            kind: ImageType::Image,
            url,
        }
    }
}
