use utoipa::OpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::routes::MastodonApi;

pub mod v1;
pub mod v2;

pub fn routes() -> OpenApiRouter {
    OpenApiRouter::with_openapi(MastodonApi::openapi())
        .routes(routes!(v1::v1))
        .routes(routes!(v2::v2))
}
