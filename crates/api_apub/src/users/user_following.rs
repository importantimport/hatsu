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
// use axum_extra::{
//     extract::Query,
//     routing::TypedPath,
// };
use hatsu_apub::collections::{Collection, CollectionPage};
use hatsu_utils::{AppData, AppError};
use serde::Deserialize;
use serde_json::Value;
use url::Url;

// #[derive(TypedPath, Deserialize)]
// #[typed_path("/u/:name/following")]
// pub struct UsersFollowing {
//     name: String
// }

// #[derive(TypedPath, Deserialize)]
// #[typed_path("/users/:name/following")]
// pub struct UsersFollowingRedirect {
//     name: String
// }

#[derive(Default, Deserialize)]
pub struct Pagination {
    page: Option<u64>,
}

#[debug_handler]
pub async fn handler(
    // UsersFollowing { name }: UsersFollowing,
    Path(name): Path<String>,
    pagination: Option<Query<Pagination>>,
    data: Data<AppData>,
) -> Result<FederationJson<WithContext<Value>>, AppError> {
    let Query(pagination) = pagination.unwrap_or_default();

    match pagination.page {
        None => Ok(FederationJson(WithContext::new_default(
            serde_json::to_value(Collection::new(
                &hatsu_utils::url::generate_user_url(data.domain(), &name)?
                    .join(&format!("{name}/following"))?,
                0,
                Some(0),
            )?)?,
        ))),
        Some(page) => Ok(FederationJson(WithContext::new_default(
            serde_json::to_value(CollectionPage::<Url>::new(
                hatsu_utils::url::generate_user_url(data.domain(), &name)?
                    .join(&format!("{name}/following"))?,
                0,
                vec![],
                0,
                page,
            )?)?,
        ))),
    }
}

#[debug_handler]
pub async fn redirect(
    // UsersFollowingRedirect { name }: UsersFollowingRedirect,
    Path(name): Path<String>,
) -> Redirect {
    Redirect::permanent(&format!("/users/{name}/followers"))
}
