use activitypub_federation::{
    axum::json::FederationJson,
    config::Data,
    protocol::context::WithContext,
    traits::Object,
};
use axum::{
    debug_handler,
    extract::Path,
    response::{IntoResponse, Redirect},
};
use sea_orm::*;

use crate::{
    AppData,
    AppError,
    entities::prelude::*,
    protocol::actors::Person
};

#[debug_handler]
pub async fn handler(
    Path(name): Path<String>,
    data: Data<AppData>,
) -> Result<FederationJson<WithContext<Person>>, AppError> {
    let id = format!("https://{}/u/{}", data.domain(), &name);

    match User::find_by_id(&id)
        .one(&data.conn)
        .await? {
            Some(user) => Ok(FederationJson(WithContext::new_default(user.into_json(&data).await?))),
            None => Err(AppError::not_found("User", &name))
        }
}

#[debug_handler]
pub async fn redirect(Path(name): Path<String>) -> impl IntoResponse {
    Redirect::permanent(&format!("/u/{}", name)).into_response()
}
