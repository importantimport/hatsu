use axum::{routing::get, Router};

mod status_context;
use status_context::status_context;

pub fn handler() -> Router {
    Router::new()
        .route("/api/v1/statuses/:status/context", get(status_context))
}
