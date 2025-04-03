use activitypub_federation::config::Data;
use axum::{Json, debug_handler};
use hatsu_utils::AppData;

use crate::{TAG, entities::NodeInfoWellKnown};

/// NodeInfo discovery.
///
/// <https://nodeinfo.diaspora.software/protocol.html>
#[utoipa::path(
    get,
    tag = TAG,
    path = "/.well-known/nodeinfo",
    responses(
        (status = OK, description = "", body = NodeInfoWellKnown),
    ),
)]
#[debug_handler]
pub async fn discovery(data: Data<AppData>) -> Json<NodeInfoWellKnown> {
    Json(NodeInfoWellKnown::new(&data))
}
