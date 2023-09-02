use activitypub_federation::{
    axum::json::FederationJson,
    config::Data, protocol::context::WithContext,
};
use axum::{
    debug_handler,
    extract::{Path, Query},
    response::{IntoResponse, Redirect},
};
use serde::Deserialize;
use serde_json::Value;
use url::Url;

use crate::{
    AppData,
    AppError,
    protocol::collections::followers::{Followers, FollowersPage},
};

#[derive(Deserialize)]
pub struct Pagination {
    page: Option<u64>,
}

impl Default for Pagination {
    fn default() -> Self {
        Self { page: None }
    }
}

#[debug_handler]
pub async fn handler(
    Path(name): Path<String>,
    pagination: Option<Query<Pagination>>,
    data: Data<AppData>,
) -> Result<FederationJson<WithContext<Value>>, AppError> {
    let Query(pagination) = pagination.unwrap_or_default();

    Ok(FederationJson(WithContext::new_default(
        match pagination.page {
            None => {
                serde_json::to_value(Followers::new(
                    Url::parse(&format!("https://{}/u/{}/followers", data.domain(), name))?,
                    0
                )?)?
            },
            Some(page) => {
                serde_json::to_value(FollowersPage::new(
                    Url::parse(&format!("https://{}/u/{}/followers", data.domain(), name))?,
                    0,
                    vec![],
                    128,
                    page
                )?)?
            }
        }
    )))
}

#[debug_handler]
pub async fn redirect(Path(name): Path<String>) -> impl IntoResponse {
    Redirect::permanent(&format!("/u/{}/followers", name)).into_response()
}
