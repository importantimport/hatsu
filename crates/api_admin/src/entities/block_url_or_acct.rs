use serde::{Deserialize, Serialize};
use url::Url;
use utoipa::{IntoParams, ToSchema};

#[derive(Deserialize, IntoParams)]
pub struct BlockUrlQuery {
    pub url: Url,
}

#[derive(Serialize, ToSchema)]
pub struct BlockUrlResult {
    pub url: Url,
    pub message: String,
}
