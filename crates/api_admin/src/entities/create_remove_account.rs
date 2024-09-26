use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Deserialize, IntoParams)]
pub struct CreateRemoveAccountQuery {
    pub name: String,
}

#[derive(Serialize, ToSchema)]
pub struct CreateRemoveAccountResult {
    pub name: String,
    pub message: String,
}
