use activitypub_federation::{
    config::Data,
    traits::Object
};
use anyhow::anyhow;
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
    entities::{
        prelude::*,
        post::Model as DbPost
    },
    protocol::objects::Note,
    utilities::remove_https
};

#[debug_handler]
pub async fn handler(
  Path(mut object): Path<String>,
  data: Data<AppData>,
) -> Result<Json<Note>, AppError> {
    object = remove_https(object);

    tracing::info!("Reading object {}", object);

    let object_id = format!("https://{}/o/{}", data.domain(), object);
    let db_post: Option<DbPost> = Post::find_by_id(object_id)
        .one(&data.conn)
        .await?;

    match db_post {
        Some(db_post) => Ok(Json(db_post.into_json(&data).await?)),
        // TODO: StatusCode::NOT_FOUND
        None => Err(AppError(anyhow!("Object Not Found")))
    }
}

#[debug_handler]
pub async fn redirect(Path(object): Path<String>) -> impl IntoResponse {
    Redirect::permanent(&format!("/o/{}", object)).into_response()
}
