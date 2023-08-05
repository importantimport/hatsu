use activitypub_federation::{
    config::Data,
    traits::Object
};
use anyhow::anyhow;
use axum::{
    Json,
    debug_handler,
    extract::Path,
};
use sea_orm::*;

use crate::{
    AppData,
    AppError,
    entities::{
        prelude::*,
        activity::Model as DbActivity
    },
    protocol::activities::activity_lists::PersonInboxActivities,
    utilities::generate_activity_id
};

#[debug_handler]
pub async fn activity(
  Path(activity): Path<String>,
  data: Data<AppData>,
) -> Result<Json<PersonInboxActivities>, AppError> {
    tracing::info!("Reading activity {}", activity);

    let activity_id = generate_activity_id(data.domain(), Some(activity))?.to_string();
    let db_activity: Option<DbActivity> = Activity::find_by_id(activity_id)
        .one(&data.conn)
        .await?;

    match db_activity {
        Some(db_activity) => Ok(Json(db_activity.into_json(&data).await?)),
        // TODO: StatusCode::NOT_FOUND
        None => Err(AppError(anyhow!("Activity Not Found")))
    }
}
