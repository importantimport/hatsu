use activitypub_federation::config::Data;
use axum::{debug_handler, Json};
use hatsu_utils::{AppData, AppError};

use crate::entities::{Instance, InstanceV1};

/// (DEPRECATED) View server information (V1)
///
/// <https://docs.joinmastodon.org/methods/instance/#v1>
#[utoipa::path(
    get,
    tag = "mastodon",
    path = "/api/v1/instance",
    responses(
        (status = OK, description = "", body = InstanceV1),
    ),
)]
#[debug_handler]
pub async fn v1(data: Data<AppData>) -> Result<Json<InstanceV1>, AppError> {
    Ok(Json(InstanceV1::from_instance(
        Instance::new(&data).await?,
    )?))
}
