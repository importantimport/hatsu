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
use hatsu_apub::{
    actors::ApubUser,
    collections::{Collection, CollectionOrPage, CollectionPage},
};
use hatsu_db_schema::{prelude::ReceivedFollow, received_follow};
use hatsu_utils::{AppData, AppError};
use sea_orm::{ModelTrait, PaginatorTrait, QueryOrder};
use url::Url;

use crate::{users::Pagination, TAG};

/// Get user followers
#[utoipa::path(
    get,
    tag = TAG,
    path = "/users/{user}/followers",
    responses(
        (status = OK, description = "Followers", body = CollectionOrPage),
        (status = NOT_FOUND, description = "User does not exist", body = AppError)
    ),
    params(
        ("user" = String, Path, description = "The Domain of the User in the database."),
        Pagination
    )
)]
#[debug_handler]
pub async fn handler(
    Path(name): Path<String>,
    pagination: Query<Pagination>,
    data: Data<AppData>,
) -> Result<FederationJson<WithContext<CollectionOrPage>>, AppError> {
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
            CollectionOrPage::Collection(Collection::new(
                &hatsu_utils::url::generate_user_url(data.domain(), &name)?
                    .join(&format!("{name}/followers"))?,
                total.number_of_items,
                Some(total.number_of_pages),
            )?),
        ))),
        Some(page) =>
            if page > 1 && page > total.number_of_pages {
                Err(AppError::not_found(
                    &format!("user {name}"),
                    &format!("followers page {page}"),
                ))
            } else {
                Ok(FederationJson(WithContext::new_default(
                    CollectionOrPage::CollectionPageUrl(CollectionPage::<Url>::new(
                        hatsu_utils::url::generate_user_url(data.domain(), &name)?
                            .join(&format!("{name}/followers"))?,
                        total.number_of_items,
                        follower_pages
                            .fetch_page(page - 1)
                            .await?
                            .into_iter()
                            .map(|follow| Url::parse(&follow.id))
                            .filter_map(Result::ok)
                            .collect(),
                        total.number_of_pages,
                        page,
                    )?),
                )))
            },
    }
}

#[debug_handler]
pub async fn redirect(Path(name): Path<String>) -> Redirect {
    Redirect::permanent(&format!("/users/{name}/followers"))
}
