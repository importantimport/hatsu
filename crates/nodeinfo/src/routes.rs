use axum::{routing::get, Router};

use crate::handler;

pub fn routes() -> Router {
    Router::new()
        .route("/nodeinfo/2.0", get(handler::v2_0))
        .route("/nodeinfo/2.0.json", get(handler::v2_0))
        .route("/nodeinfo/2.1", get(handler::v2_1))
        .route("/nodeinfo/2.1.json", get(handler::v2_1))
}
