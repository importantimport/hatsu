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
use serde_json::{from_str, Value};

use crate::{
    AppData,
    AppError,
    entities::{
        prelude::*,
        activity::Model as DbActivity
    },
};

#[debug_handler]
pub async fn activity(
  Path(activity_id): Path<String>,
  data: Data<AppData>,
) -> Result<FederationJson<Value>, AppError> {
    tracing::info!("Reading activity {}", activity_id);

    let db_activity: Option<DbActivity> = Activity::find_by_id(activity_id)
        .one(&data.conn)
        .await?;

    match db_activity {
        Some(db_activity) => Ok(FederationJson(from_str(&db_activity.activity)?)),
        // TODO: StatusCode::NOT_FOUND
        None => Err(AppError(anyhow!("Activity Not Found")))
    }
}
