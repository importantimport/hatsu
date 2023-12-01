use activitypub_federation::{
    config::Data,
    protocol::context::WithContext,
    traits::Object,
};
use axum::{
    Json,
    debug_handler,
    response::{Redirect, IntoResponse},
};
use axum_extra::routing::TypedPath;
use sea_orm::*;
use serde::Deserialize;

use crate::{
    AppData,
    AppError,
    entities::prelude::*,
    protocol::objects::Note,
};

#[derive(TypedPath, Deserialize)]
#[typed_path("/o/*object")]
pub struct Objects {
    object: String
}

#[derive(TypedPath, Deserialize)]
#[typed_path("/objects/*object")]
pub struct ObjectsRedirect {
    object: String
}

#[debug_handler]
pub async fn handler(
    Objects { object }: Objects,
    data: Data<AppData>,
) -> Result<Json<WithContext<Note>>, AppError> {
    tracing::info!("Reading object {}", object);

    let object_id = format!("https://{}/o/{}", data.domain(), object);

    match Post::find_by_id(&object_id)
        .one(&data.conn)
        .await? {
            Some(db_post) => Ok(Json(WithContext::new_default(db_post.into_json(&data).await?))),
            None => Err(AppError::not_found("Object", &object_id))
        }
}

#[debug_handler]
pub async fn redirect(
    ObjectsRedirect { object }: ObjectsRedirect
) -> impl IntoResponse {
    Redirect::permanent(&format!("/o/{}", object)).into_response()
}
