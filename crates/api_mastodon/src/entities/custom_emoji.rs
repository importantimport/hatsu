use serde::{Deserialize, Serialize};
use url::Url;
use utoipa::ToSchema;

/// https://docs.joinmastodon.org/entities/CustomEmoji/
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct CustomEmoji {
    shortcode: String,
    url: Url,
    static_url: Url,
    visible_in_picker: bool,
    category: Option<String>,
}
