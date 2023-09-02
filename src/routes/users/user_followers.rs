use activitypub_federation::{
    axum::json::FederationJson,
    config::Data, protocol::context::WithContext, fetch::object_id::ObjectId,
};
use axum::{
    debug_handler,
    extract::{Path, Query},
    response::{IntoResponse, Redirect},
};
use sea_orm::*;
use serde::Deserialize;
use serde_json::Value;
use url::Url;

use crate::{
    AppData,
    AppError,
    entities::{
        prelude::*,
        received_follow,
        user::Model as DbUser,
    },
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

    let user_id: ObjectId<DbUser> = Url::parse(&format!("https://{}/u/{}", data.domain(), &name))?.into();
    let user = user_id.dereference_local(&data).await?;

    let follower_pages = user.find_related(ReceivedFollow)
        .order_by_asc(received_follow::Column::Id)
        .paginate(&data.conn, 12);

    let total = follower_pages.num_items_and_pages().await?;

    Ok(FederationJson(WithContext::new_default(
        match pagination.page {
            None => {
                serde_json::to_value(Followers::new(
                    Url::parse(&format!("https://{}/u/{}/followers", data.domain(), name))?,
                    total.number_of_items,
                )?)?
            },
            Some(page) => {
                serde_json::to_value(FollowersPage::new(
                    Url::parse(&format!("https://{}/u/{}/followers", data.domain(), name))?,
                    total.number_of_items,
                    follower_pages
                        .fetch_page(page - 1)
                        .await?
                        .into_iter()
                        .map(|follow| Url::parse(&follow.id).unwrap())
                        .collect(),
                    total.number_of_pages,
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
