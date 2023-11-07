use activitypub_federation::{
    config::Data,
    protocol::context::WithContext,
    traits::Object,
};
use axum::{
    Json,
    debug_handler,
    extract::Path,
    response::{Redirect, IntoResponse},
};
use sea_orm::*;

use crate::{
    AppData,
    AppError,
    entities::prelude::*,
    protocol::objects::Note,
};

#[debug_handler]
pub async fn handler(
  Path(object): Path<String>,
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
pub async fn redirect(Path(object): Path<String>) -> impl IntoResponse {
    Redirect::permanent(&format!("/o/{}", object)).into_response()
}
