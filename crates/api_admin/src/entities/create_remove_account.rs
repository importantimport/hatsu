use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct CreateRemoveAccount {
    pub name: String,
}

#[derive(Serialize, ToSchema)]
pub struct CreateRemoveAccountResult {
    pub name: String,
    pub message: String,
}
