use activitypub_federation::{
    axum::json::FederationJson,
    config::Data,
    protocol::context::WithContext,
};
use axum::{
    debug_handler,
    extract::{Path, Query},
    response::Redirect,
};
use hatsu_apub::collections::{Collection, CollectionOrPage, CollectionPage};
use hatsu_utils::{AppData, AppError};

use crate::{TAG, users::Pagination};

/// Get user following
#[utoipa::path(
    get,
    tag = TAG,
    path = "/users/{user}/following",
    responses(
        (status = OK, description = "Following", body = CollectionOrPage),
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
    match pagination.page {
        None => Ok(FederationJson(WithContext::new_default(
            CollectionOrPage::Collection(Collection::new(
                &hatsu_utils::url::generate_user_url(data.domain(), &name)?
                    .join(&format!("{name}/following"))?,
                0,
                0,
            )?),
        ))),
        Some(page) => Ok(FederationJson(WithContext::new_default(
            CollectionOrPage::CollectionPage(CollectionPage::new(
                hatsu_utils::url::generate_user_url(data.domain(), &name)?
                    .join(&format!("{name}/following"))?,
                0,
                vec![],
                0,
                page,
            )?),
        ))),
    }
}

#[debug_handler]
pub async fn redirect(Path(name): Path<String>) -> Redirect {
    Redirect::permanent(&format!("/users/{name}/followers"))
}
