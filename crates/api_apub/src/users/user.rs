use activitypub_federation::{
    axum::json::FederationJson,
    config::Data,
    kinds::{context, security},
    protocol::context::WithContext,
    traits::Object,
};
use axum::{debug_handler, extract::Path, response::Redirect};
// use axum_extra::routing::TypedPath;
use hatsu_apub::actors::{ApubUser, User};
use hatsu_db_schema::prelude::User as PreludeUser;
use hatsu_utils::{AppData, AppError};
use sea_orm::EntityTrait;
// use serde::Deserialize;
use serde_json::{json, Value};

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

/// Get user
#[utoipa::path(
    get,
    tag = "apub",
    path = "/users/{user}",
    responses(
        (status = OK, description = "User", body = User),
        (status = NOT_FOUND, description = "User does not exist", body = AppError)
    ),
    params(
        ("user" = String, Path, description = "The Domain of the User in the database.")
    )
)]
#[debug_handler]
pub async fn user(
    // Users { name }: Users,
    Path(name): Path<String>,
    data: Data<AppData>,
) -> Result<FederationJson<WithContext<User>>, AppError> {
    let url = hatsu_utils::url::generate_user_url(data.domain(), &name)?;
    // "@context": [
    //   "https://www.w3.org/ns/activitystreams",
    //   "https://w3id.org/security/v1",
    //   "https://purl.archive.org/socialweb/webfinger",
    //   {
    //     "xrd": "http://docs.oasis-open.org/ns/xri/xrd-1.0#",
    //     "aliases": {
    //       "@id": "xrd:Alias",
    //       "@type": "@id",
    //       "@container": "@list"
    //     }
    //   }
    // ]
    let context = vec![
        Value::String(context().to_string()),
        Value::String(security().to_string()),
        // FEP-2c59
        Value::String(String::from("https://purl.archive.org/socialweb/webfinger")),
        // FEP-4adb
        json!({
            "xrd": "http://docs.oasis-open.org/ns/xri/xrd-1.0#",
            "aliases": {
              "@id": "xrd:Alias",
              "@type": "@id",
              "@container": "@list"
            },
        }),
    ];

    match PreludeUser::find_by_id(&url.to_string())
        .one(&data.conn)
        .await?
    {
        Some(db_user) => {
            let apub_user: ApubUser = db_user.into();
            Ok(FederationJson(WithContext::new(
                apub_user.into_json(&data).await?,
                Value::Array(context),
            )))
        },
        None => Err(AppError::not_found("User", &name)),
    }
}

#[debug_handler]
pub async fn redirect(
    // UsersRedirect { name }: UsersRedirect,
    Path(name): Path<String>,
) -> Redirect {
    Redirect::permanent(&format!("/users/{name}"))
}
