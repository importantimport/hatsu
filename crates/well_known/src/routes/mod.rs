use utoipa::OpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::entities::{
    HostMeta,
    HostMetaLink,
    NodeInfoWellKnown,
    NodeInfoWellKnownLink,
    WebfingerSchema,
    WebfingerSchemaLink,
};

mod host_meta;
mod nodeinfo;
mod webfinger;

pub const TAG: &str = "well_known";

#[derive(OpenApi)]
#[openapi(
    components(schemas(
        HostMeta,
        HostMetaLink,
        NodeInfoWellKnown,
        NodeInfoWellKnownLink,
        WebfingerSchema,
        WebfingerSchemaLink,
    )),
    tags((name = TAG, description = "Well Known (/.well-known/)"))
)]
pub struct WellKnownApi;

pub fn routes() -> OpenApiRouter {
    OpenApiRouter::with_openapi(WellKnownApi::openapi())
        .routes(routes!(host_meta::redirect))
        .routes(routes!(host_meta::xml))
        .routes(routes!(host_meta::json))
        .routes(routes!(nodeinfo::discovery))
        .routes(routes!(webfinger::webfinger))
}
