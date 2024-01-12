use activitypub_federation::{axum::json::FederationJson, config::Data};
use axum::{
    debug_handler,
    extract::Path,
    response::{IntoResponse, Redirect},
};
// use axum_extra::routing::TypedPath;
use hatsu_apub::activities::ApubActivity;
use hatsu_db_schema::prelude::Activity;
use hatsu_utils::{AppData, AppError};
use sea_orm::EntityTrait;
// use serde::Deserialize;
use serde_json::Value;

// #[derive(TypedPath, Deserialize)]
// #[typed_path("/a/*activity_id")]
// pub struct Activities {
//     activity_id: String
// }

// #[derive(TypedPath, Deserialize)]
// #[typed_path("/activities/*activity_id")]
// pub struct ActivitiesRedirect {
//     activity_id: String
// }

#[debug_handler]
pub async fn handler(
    // Activities { activity_id }: Activities,
    Path(activity_id): Path<String>,
    data: Data<AppData>,
) -> Result<FederationJson<Value>, AppError> {
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
            Ok(FederationJson(activity.into_json()?))
        }
        None => Err(AppError::not_found("Activity", &activity_id)),
    }
}

#[debug_handler]
pub async fn redirect(
    // ActivitiesRedirect { activity_id }: ActivitiesRedirect,
    Path(activity_id): Path<String>,
) -> impl IntoResponse {
    Redirect::permanent(&format!("/a/{activity_id}")).into_response()
}
