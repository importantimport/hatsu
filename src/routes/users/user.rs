use activitypub_federation::{
    axum::json::FederationJson,
    config::Data,
    kinds::{context, security},
    protocol::context::WithContext,
    traits::Object,
};
use axum::{
    debug_handler,
    extract::Path,
    response::{IntoResponse, Redirect},
};
// use axum_extra::routing::TypedPath;
use hatsu_apub::actors::{ApubUser, Service};
use hatsu_db_schema::prelude::*;
use hatsu_utils::{AppData, AppError};
use sea_orm::*;
// use serde::Deserialize;
use serde_json::Value;

// #[derive(TypedPath, Deserialize)]
// #[typed_path("/u/:name")]
// pub struct Users {
//     name: String
// }

// #[derive(TypedPath, Deserialize)]
// #[typed_path("/users/:name")]
// pub struct UsersRedirect {
//     name: String
// }

#[debug_handler]
pub async fn handler(
    // Users { name }: Users,
    Path(name): Path<String>,
    data: Data<AppData>,
) -> Result<FederationJson<WithContext<Service>>, AppError> {
    let id = format!("https://{}/u/{}", data.domain(), &name);
    // "@context": [
    //   "https://www.w3.org/ns/activitystreams",
    //   "https://w3id.org/security/v1"
    // ]
    let context = vec![Value::String(context().to_string()), Value::String(security().to_string())];

    match User::find_by_id(&id)
        .one(&data.conn)
        .await? {
            Some(db_user) => {
                let apub_user: ApubUser = db_user.into();
                Ok(FederationJson(WithContext::new(apub_user.into_json(&data).await?, context)))
            },
            None => Err(AppError::not_found("User", &name))
        }
}

#[debug_handler]
pub async fn redirect(
    // UsersRedirect { name }: UsersRedirect,
    Path(name): Path<String>,
) -> impl IntoResponse {
    Redirect::permanent(&format!("/u/{}", name)).into_response()
}
