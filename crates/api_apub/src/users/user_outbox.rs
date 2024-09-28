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
    activities::ApubActivity,
    actors::ApubUser,
    collections::{Collection, CollectionOrPage, CollectionPage},
};
use hatsu_db_schema::{activity, prelude::Activity};
use hatsu_utils::{AppData, AppError};
use sea_orm::{ColumnTrait, ModelTrait, PaginatorTrait, QueryFilter, QueryOrder};
use serde_json::Value;

use crate::{users::Pagination, TAG};

/// Get user outbox
#[utoipa::path(
    get,
    tag = TAG,
    path = "/users/{user}/outbox",
    responses(
        (status = OK, description = "Outbox", body = CollectionOrPage),
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

    let activity_pages = user
        .find_related(Activity)
        .filter(activity::Column::Kind.eq("Create"))
        // TODO: order by last_refreshed_at
        .order_by_desc(activity::Column::Published)
        // .order_by_asc(activity::Column::Id)
        // 20 per page
        .paginate(&data.conn, 20);

    let total = activity_pages.num_items_and_pages().await?;

    match pagination.page {
        None => Ok(FederationJson(WithContext::new_default(
            CollectionOrPage::Collection(Collection::new(
                &hatsu_utils::url::generate_user_url(data.domain(), &name)?
                    .join(&format!("{name}/outbox"))?,
                total.number_of_items,
                total.number_of_pages,
            )?),
        ))),
        Some(page) =>
            if page > 1 && page > total.number_of_pages {
                Err(AppError::not_found(
                    &format!("user {name}"),
                    &format!("outbox page {page}"),
                ))
            } else {
                Ok(FederationJson(WithContext::new_default(
                    CollectionOrPage::CollectionPageValue(CollectionPage::<Value>::new(
                        hatsu_utils::url::generate_user_url(data.domain(), &name)?
                            .join(&format!("{name}/outbox"))?,
                        total.number_of_items,
                        activity_pages
                            .fetch_page(page - 1)
                            .await?
                            .into_iter()
                            .map(|activity| {
                                let activity: ApubActivity = activity.into();
                                activity.into_json()
                            })
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
    Redirect::permanent(&format!("/users/{name}/outbox"))
}
