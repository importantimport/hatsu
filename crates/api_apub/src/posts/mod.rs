use axum::routing::get;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;

use crate::ApubApi;

pub mod notice;
pub mod post;

pub fn routes() -> OpenApiRouter {
    OpenApiRouter::with_openapi(ApubApi::openapi())
        .route("/notice/*notice", get(notice::notice))
        .route("/posts/*post", get(post::post))
        .route("/p/*post", get(post::redirect))
}
