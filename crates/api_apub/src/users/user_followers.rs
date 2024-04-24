use activitypub_federation::{
    axum::json::FederationJson,
    config::Data,
    fetch::object_id::ObjectId,
    protocol::context::WithContext,
};
use axum::{
    debug_handler,
    extract::{Path, Query},
    response::Redirect,
};
// use axum_extra::{
//     extract::Query,
//     routing::TypedPath,
// };
use hatsu_apub::{
    actors::ApubUser,
    collections::{Collection, CollectionPage},
};
use hatsu_db_schema::{prelude::ReceivedFollow, received_follow};
use hatsu_utils::{AppData, AppError};
use sea_orm::{ModelTrait, PaginatorTrait, QueryOrder};
use serde::Deserialize;
use serde_json::Value;
use url::Url;

// #[derive(TypedPath, Deserialize)]
// #[typed_path("/u/:name/followers")]
// pub struct UsersFollowers {
//     name: String
// }

// #[derive(TypedPath, Deserialize)]
// #[typed_path("/users/:name/followers")]
// pub struct UsersFollowersRedirect {
//     name: String
// }

#[derive(Default, Deserialize)]
pub struct Pagination {
    page: Option<u64>,
}

#[debug_handler]
pub async fn handler(
    // UsersFollowers { name }: UsersFollowers,
    Path(name): Path<String>,
    pagination: Option<Query<Pagination>>,
    data: Data<AppData>,
) -> Result<FederationJson<WithContext<Value>>, AppError> {
    let Query(pagination) = pagination.unwrap_or_default();

    let user_id: ObjectId<ApubUser> =
        hatsu_utils::url::generate_user_url(data.domain(), &name)?.into();
    let user = user_id.dereference_local(&data).await?;

    let follower_pages = user
        .find_related(ReceivedFollow)
        // TODO: order by last_refreshed_at
        .order_by_asc(received_follow::Column::Id)
        // 12 per page
        .paginate(&data.conn, 12);

    let total = follower_pages.num_items_and_pages().await?;

    match pagination.page {
        None => Ok(FederationJson(WithContext::new_default(
            serde_json::to_value(Collection::new(
                &hatsu_utils::url::generate_user_url(data.domain(), &name)?
                    .join(&format!("{name}/followers"))?,
                total.number_of_items,
                Some(total.number_of_pages),
            )?)?,
        ))),
        Some(page) =>
            if page > 1 && page > total.number_of_pages {
                Err(AppError::not_found(
                    &format!("user {name}"),
                    &format!("followers page {page}"),
                ))
            } else {
                Ok(FederationJson(WithContext::new_default(
                    serde_json::to_value(CollectionPage::<Url>::new(
                        hatsu_utils::url::generate_user_url(data.domain(), &name)?
                            .join(&format!("{name}/followers"))?,
                        total.number_of_items,
                        follower_pages
                            .fetch_page(page - 1)
                            .await?
                            .into_iter()
                            .map(|follow| Url::parse(&follow.id).unwrap())
                            .collect(),
                        total.number_of_pages,
                        page,
                    )?)?,
                )))
            },
    }
}

#[debug_handler]
pub async fn redirect(
    // UsersFollowersRedirect { name }: UsersFollowersRedirect,
    Path(name): Path<String>,
) -> Redirect {
    Redirect::permanent(&format!("/users/{name}/followers"))
}
