use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// https://docs.joinmastodon.org/entities/Account/
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct Account {
    // TODO
}
