use axum::{routing::get, Router};

pub mod status_context;
use status_context::status_context;

pub mod status_favourited_by;
use status_favourited_by::status_favourited_by;

pub mod status_reblogged_by;
use status_reblogged_by::status_reblogged_by;

pub fn routes() -> Router {
    Router::new()
        .route("/api/v1/statuses/:status/context", get(status_context))
        .route(
            "/api/v1/statuses/:status/favourited_by",
            get(status_favourited_by),
        )
        .route(
            "/api/v1/statuses/:status/reblogged_by",
            get(status_reblogged_by),
        )
}
