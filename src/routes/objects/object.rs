use activitypub_federation::{
    config::Data,
    // traits::Object
};
use axum::{
    // Json,
    debug_handler,
    extract::Path,
//   response::IntoResponse,
//   http::StatusCode
};
// use sea_orm::*;

use crate::{
  AppData,
//   entities::{
//     prelude::*,
//     post::Model as DbPost
//   },
  error::Error
};

#[debug_handler]
pub async fn object(
  Path(object): Path<String>,
  _data: Data<AppData>,
) -> Result<(), Error> {
    tracing::info!("Reading object {}", object);
    // let object_id = format!("https://{}/o/{}", data.domain(), object);
    // let db_post: DbPost = Post::find_by_id(object_id)
    //     .one(&data.conn)
    //     .await?
    //     .unwrap();

    // TODO: db_post.into_json()
    Ok(())

    // (StatusCode::OK, db_post.into_json(data))
}
