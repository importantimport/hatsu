use activitypub_federation::{
    axum::json::FederationJson,
    config::Data,
};
use axum::{
    debug_handler,
    extract::Path,
    response::{IntoResponse, Redirect},
};
use sea_orm::*;
use serde_json::Value;

use crate::{
    AppData,
    AppError,
    entities::prelude::*,
    utilities::generate_activity_url
};

#[debug_handler]
pub async fn handler(
  Path(activity_id): Path<String>,
  data: Data<AppData>,
) -> Result<FederationJson<Value>, AppError> {
    tracing::info!("Reading activity {}", activity_id);

    match Activity::find_by_id(generate_activity_url(data.domain(), Some(activity_id.clone()))?)
        .one(&data.conn)
        .await? {
            Some(activity) => Ok(FederationJson(activity.into_json()?)),
            None => Err(AppError::not_found("Activity", &activity_id))
        }
}

#[debug_handler]
pub async fn redirect(Path(activity_id): Path<String>) -> impl IntoResponse {
    Redirect::permanent(&format!("/a/{}", activity_id)).into_response()
}
