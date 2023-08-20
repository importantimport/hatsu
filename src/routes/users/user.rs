use activitypub_federation::{
    axum::json::FederationJson,
    config::Data,
    protocol::context::WithContext,
    traits::Object,
};
use anyhow::anyhow;
use axum::{
    debug_handler,
    extract::Path,
    response::{IntoResponse, Redirect},
};
use sea_orm::*;

use crate::{
    AppData,
    AppError,
    entities::{
        prelude::*,
        user::Model as DbUser,
    },
    protocol::actors::Person
};

#[debug_handler]
pub async fn handler(
    Path(name): Path<String>,
    data: Data<AppData>,
) -> Result<FederationJson<WithContext<Person>>, AppError> {
    let id = format!("https://{}/u/{}", data.domain(), &name);
    let user: Option<DbUser> = User::find_by_id(&id)
        .one(&data.conn)
        .await?;

    match user {
        Some(user) => Ok(FederationJson(WithContext::new_default(user.into_json(&data).await?))),
        // TODO: StatusCode::NOT_FOUND
        None => Err(AppError(anyhow!("User Not Found")))
    }
}

#[debug_handler]
pub async fn redirect(Path(name): Path<String>) -> impl IntoResponse {
    Redirect::permanent(&format!("/u/{}", name)).into_response()
}
