use axum::{routing::get, Router};

pub mod v1;
pub mod v2;

pub fn routes() -> Router {
    Router::new()
        .route("/api/v1/instance", get(v1::v1))
        .route("/api/v2/instance", get(v2::v2))
}
