use activitypub_federation::{
    axum::json::FederationJson,
    config::Data, protocol::context::WithContext,
    fetch::object_id::ObjectId,
};
use axum::{
    debug_handler,
    response::{IntoResponse, Redirect},
};
use axum_extra::{
    extract::Query,
    routing::TypedPath,
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
        activity,
        user::Model as DbUser,
    },
    protocol::collections::{CollectionPage, Collection},
};

#[derive(TypedPath, Deserialize)]
#[typed_path("/u/:name/outbox")]
pub struct UsersOutbox {
    name: String
}

#[derive(TypedPath, Deserialize)]
#[typed_path("/users/:name/outbox")]
pub struct UsersOutboxRedirect {
    name: String
}

#[derive(Default, Deserialize)]
pub struct Pagination {
    page: Option<u64>,
}

#[debug_handler]
pub async fn handler(
    UsersOutbox { name }: UsersOutbox,
    pagination: Option<Query<Pagination>>,
    data: Data<AppData>,
) -> Result<FederationJson<WithContext<Value>>, AppError> {
    let Query(pagination) = pagination.unwrap_or_default();

    let user_id: ObjectId<DbUser> = Url::parse(&format!("https://{}/u/{}", data.domain(), &name))?.into();
    let user = user_id.dereference_local(&data).await?;

    let activity_pages = user.find_related(Activity)
        .filter(activity::Column::Kind.eq("Create"))
        // TODO: order by last_refreshed_at
        .order_by_desc(activity::Column::Published)
        // .order_by_asc(activity::Column::Id)
        // 20 per page
        .paginate(&data.conn, 20);

    let total = activity_pages.num_items_and_pages().await?;

    match pagination.page {
        None => {
            Ok(FederationJson(WithContext::new_default(
                serde_json::to_value(Collection::new(
                    Url::parse(&format!("https://{}/u/{}/outbox", data.domain(), name))?,
                    total.number_of_items,
                    Some(total.number_of_pages),
                )?)?
            )))
        },
        Some(page) => {
            if page > 1 && page > total.number_of_pages {
                Err(AppError::not_found(
                    &format!("user {}", name),
                    &format!("outbox page {}", page) )
                )
            } else {
                Ok(FederationJson(WithContext::new_default(
                    serde_json::to_value(CollectionPage::<Value>::new(
                        Url::parse(&format!("https://{}/u/{}/outbox", data.domain(), name))?,
                        total.number_of_items,
                        activity_pages
                            .fetch_page(page - 1)
                            .await?
                            .into_iter()
                            .map(|activity| activity.into_json().unwrap())
                            .collect(),
                        total.number_of_pages,
                        page
                    )?)?
                )))
            }
        }
    }
}

#[debug_handler]
pub async fn redirect(
    UsersOutboxRedirect { name }: UsersOutboxRedirect,
) -> impl IntoResponse {
    Redirect::permanent(&format!("/u/{}/outbox", name)).into_response()
}
