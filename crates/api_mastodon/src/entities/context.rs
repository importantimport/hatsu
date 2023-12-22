use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::entities::Status;

/// https://docs.joinmastodon.org/entities/Context/
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct Context {
    /// should always be empty vec
    pub ancestors: Vec<Status>,
    pub descendants: Vec<Status>,
}
