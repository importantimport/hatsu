use activitypub_federation::{
    axum::json::FederationJson,
    config::Data,
};
use anyhow::anyhow;
use axum::{
    debug_handler,
    extract::Path,
};
use sea_orm::*;
use serde_json::Value;

use crate::{
    AppData,
    AppError,
    entities::prelude::*,
};

#[debug_handler]
pub async fn activity(
  Path(activity_id): Path<String>,
  data: Data<AppData>,
) -> Result<FederationJson<Value>, AppError> {
    tracing::info!("Reading activity {}", activity_id);

    match Activity::find_by_id(activity_id)
        .one(&data.conn)
        .await? {
            Some(activity) => Ok(FederationJson(activity.into_json()?)),
            // TODO: StatusCode::NOT_FOUND
            None => Err(AppError(anyhow!("Activity Not Found")))
        }
}
