use activitypub_federation::{
    axum::json::FederationJson,
    config::Data, protocol::context::WithContext,
};
use axum::{
    debug_handler,
    extract::{Path, Query},
    response::{IntoResponse, Redirect},
};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{
    AppData,
    AppError,
    protocol::collections::followers::{Followers, FollowersPage},
};

#[derive(Deserialize, Serialize)]
pub struct FollowersQuery {
    page: Option<u64>,
}

#[derive(Deserialize, Serialize)]
pub enum FollowersJson {
    Followers(Followers),
    FollowersPage(FollowersPage),
}

#[debug_handler]
pub async fn handler(
    Path(user_id): Path<String>,
    Query(query): Query<FollowersQuery>,
    data: Data<AppData>,
) -> Result<FederationJson<WithContext<FollowersJson>>, AppError> {

    Ok(FederationJson(WithContext::new_default(
        match query.page {
            None => {
                FollowersJson::Followers(Followers::new(
                    Url::parse(&format!("https://{}/u/{}/followers", data.domain(), user_id))?,
                    0
                )?)
            },
            Some(_page) => {
                FollowersJson::Followers(Followers::new(
                    Url::parse(&format!("https://{}/u/{}/followers", data.domain(), user_id))?,
                    0
                )?) 
            }
        }
    )))
}

#[debug_handler]
pub async fn redirect(Path(name): Path<String>) -> impl IntoResponse {
    Redirect::permanent(&format!("/u/{}/followers", name)).into_response()
}
