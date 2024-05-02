use activitypub_federation::config::Data;
use axum::{debug_handler, Json};
use hatsu_utils::{AppData, AppError};

use crate::schema::NodeInfo;

/// NodeInfo schema version 2.0.
///
/// <https://nodeinfo.diaspora.software/schema.html#/ns/schema/2.0#>
#[utoipa::path(
    get,
    tag = "nodeinfo",
    path = "/nodeinfo/2.0.json",
    responses(
        (status = OK, description = "", body = NodeInfo),
    ),
)]
#[debug_handler]
pub async fn v2_0(data: Data<AppData>) -> Result<Json<NodeInfo>, AppError> {
    Ok(Json(NodeInfo::v2_0(&data).await?))
}

/// NodeInfo schema version 2.1.
///
/// <https://nodeinfo.diaspora.software/schema.html#/ns/schema/2.1#>
#[utoipa::path(
    get,
    tag = "nodeinfo",
    path = "/nodeinfo/2.1.json",
    responses(
        (status = OK, description = "", body = NodeInfo),
    ),
)]
#[debug_handler]
pub async fn v2_1(data: Data<AppData>) -> Result<Json<NodeInfo>, AppError> {
    Ok(Json(NodeInfo::v2_1(&data).await?))
}
