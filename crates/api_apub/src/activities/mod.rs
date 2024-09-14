use axum::routing::get;
use utoipa::OpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::ApubApi;

mod activity;

pub fn routes() -> OpenApiRouter {
    OpenApiRouter::with_openapi(ApubApi::openapi())
        .routes(routes!(activity::activity))
        .route("/a/:activity", get(activity::redirect))
}
