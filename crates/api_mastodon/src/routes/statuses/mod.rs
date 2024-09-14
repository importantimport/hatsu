use utoipa::OpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::routes::MastodonApi;

mod status_context;
mod status_favourited_by;
mod status_reblogged_by;

pub fn routes() -> OpenApiRouter {
    OpenApiRouter::with_openapi(MastodonApi::openapi())
        .routes(routes!(status_context::status_context))
        .routes(routes!(status_favourited_by::status_favourited_by))
        .routes(routes!(status_reblogged_by::status_reblogged_by))
}
