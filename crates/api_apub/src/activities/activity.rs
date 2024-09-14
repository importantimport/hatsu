use activitypub_federation::{
    axum::json::FederationJson,
    config::Data,
    protocol::context::WithContext,
};
use axum::{debug_handler, extract::Path, response::Redirect};
use hatsu_apub::activities::ApubActivity;
use hatsu_db_schema::prelude::Activity;
use hatsu_utils::{AppData, AppError};
use sea_orm::EntityTrait;
use serde_json::Value;

use crate::TAG;

/// Get activity
#[utoipa::path(
    get,
    tag = TAG,
    path = "/activities/{activity}",
    responses(
        (status = OK, description = "Activity", body = Value),
        (status = NOT_FOUND, description = "Activity does not exist", body = AppError)
    ),
    params(
        ("activity" = String, Path, description = "The Uuid of the Activity in the database.")
    )
)]
#[debug_handler]
pub async fn activity(
    Path(activity_id): Path<String>,
    data: Data<AppData>,
) -> Result<FederationJson<WithContext<Value>>, AppError> {
    tracing::info!("Reading activity {}", activity_id);

    match Activity::find_by_id(hatsu_utils::url::generate_activity_url(
        data.domain(),
        Some(activity_id.clone()),
    )?)
    .one(&data.conn)
    .await?
    {
        Some(activity) => {
            let activity: ApubActivity = activity.into();
            Ok(FederationJson(WithContext::new_default(
                activity.into_json()?,
            )))
        },
        None => Err(AppError::not_found("Activity", &activity_id)),
    }
}

#[debug_handler]
pub async fn redirect(Path(activity_id): Path<String>) -> Redirect {
    Redirect::permanent(&format!("/activities/{activity_id}"))
}
