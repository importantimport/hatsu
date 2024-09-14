use activitypub_federation::config::Data;
use axum::{debug_handler, Json};
use hatsu_utils::AppData;

use crate::{entities::NodeInfoWellKnown, TAG};

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
