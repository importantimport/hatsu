use activitypub_federation::{
    config::Data,
    traits::Object
};
use axum::{
    Json,
    debug_handler,
    extract::Path,
};
use sea_orm::*;

use crate::{
  AppData,
  entities::{
    prelude::*,
    post::Model as DbPost
  },
  error::Error,
  objects::post::Note
};

#[debug_handler]
pub async fn object(
  Path(object): Path<String>,
  data: Data<AppData>,
) -> Result<Json<Note>, Error> {
    tracing::info!("Reading object {}", object);

    let object_id = format!("https://{}/o/{}", data.domain(), object);
    let db_post: DbPost = Post::find_by_id(object_id)
        .one(&data.conn)
        .await?
        .unwrap();
    let json_post: Note = db_post.into_json(&data).await?;

    Ok(Json(json_post))
}
