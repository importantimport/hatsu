use axum::routing::get;
use utoipa::OpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    handler,
    schema::{
        NodeInfo,
        NodeInfoMetadata,
        NodeInfoServices,
        NodeInfoSoftware,
        NodeInfoUsage,
        NodeInfoUsers,
    },
};

pub const TAG: &str = "nodeinfo";

#[derive(OpenApi)]
#[openapi(
    components(schemas(
        NodeInfo,
        NodeInfoMetadata,
        NodeInfoServices,
        NodeInfoSoftware,
        NodeInfoUsage,
        NodeInfoUsers,
    )),
    tags((name = TAG, description = "NodeInfo (/nodeinfo/)"))
)]
pub struct NodeInfoApi;

pub fn routes() -> OpenApiRouter {
    OpenApiRouter::with_openapi(NodeInfoApi::openapi())
        .routes(routes!(handler::v2_0))
        .routes(routes!(handler::v2_1))
        // fallback routes
        .route("/nodeinfo/2.0", get(handler::v2_0))
        .route("/nodeinfo/2.1", get(handler::v2_1))
}
