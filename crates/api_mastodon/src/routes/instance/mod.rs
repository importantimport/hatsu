use axum::{routing::get, Router};

pub mod instance;

pub fn routes() -> Router {
    Router::new()
        .route("/api/v1/instance", get(instance::v1))
        .route("/api/v2/instance", get(instance::v2))
}
