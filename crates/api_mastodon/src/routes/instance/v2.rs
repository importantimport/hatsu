use activitypub_federation::config::Data;
use axum::{debug_handler, Json};
use hatsu_utils::{AppData, AppError};

use crate::{entities::Instance, TAG};

/// View server information
///
/// <https://docs.joinmastodon.org/methods/instance/#v2>
#[utoipa::path(
    get,
    tag = TAG,
    path = "/api/v2/instance",
    responses(
        (status = OK, description = "", body = Instance),
    ),
)]
#[debug_handler]
pub async fn v2(data: Data<AppData>) -> Result<Json<Instance>, AppError> {
    Ok(Json(Instance::new(&data).await?))
}
