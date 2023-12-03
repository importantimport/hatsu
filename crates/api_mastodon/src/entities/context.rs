use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// https://docs.joinmastodon.org/entities/Context/
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct Context {
    // TODO: Vec<Status>
    // should always be empty vec
    pub ancestors: Vec<String>,
    // TODO: Vec<Status>
    pub descendants: Vec<String>,
}
