use activitypub_federation::{config::Data, protocol::context::WithContext, traits::Object};
use axum::{
    debug_handler,
    extract::Path,
    response::{IntoResponse, Redirect},
    Json,
};
// use axum_extra::routing::TypedPath;
use hatsu_apub::objects::{ApubPost, Note};
use hatsu_db_schema::prelude::Post;
use hatsu_utils::{AppData, AppError};
use sea_orm::EntityTrait;
// use serde::Deserialize;

// #[derive(TypedPath, Deserialize)]
// #[typed_path("/o/*object")]
// pub struct Objects {
//     object: String
// }

// #[derive(TypedPath, Deserialize)]
// #[typed_path("/objects/*object")]
// pub struct ObjectsRedirect {
//     object: String
// }

#[debug_handler]
pub async fn handler(
    // Objects { object }: Objects,
    Path(object): Path<String>,
    data: Data<AppData>,
) -> Result<Json<WithContext<Note>>, AppError> {
    tracing::info!("Reading object {}", object);

    let object_url = hatsu_utils::url::generate_object_url(data.domain(), object)?;

    match Post::find_by_id(&object_url.to_string())
        .one(&data.conn)
        .await?
    {
        Some(db_post) => {
            let apub_post: ApubPost = db_post.into();
            Ok(Json(WithContext::new_default(
                apub_post.into_json(&data).await?,
            )))
        }
        None => Err(AppError::not_found("Object", object_url.as_ref())),
    }
}

#[debug_handler]
pub async fn redirect(
    // ObjectsRedirect { object }: ObjectsRedirect
    Path(object): Path<String>,
) -> impl IntoResponse {
    Redirect::permanent(&format!("/o/{object}")).into_response()
}
