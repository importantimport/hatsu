use axum::{routing::get, Router};

mod nodeinfo;
mod well_known;

pub fn routes() -> Router {
    Router::new()
        .route("/.well-known/nodeinfo", get(well_known::nodeinfo))
        .route("/nodeinfo/2.0", get(nodeinfo::v2_0))
        .route("/nodeinfo/2.0.json", get(nodeinfo::v2_0))
        .route("/nodeinfo/2.1", get(nodeinfo::v2_1))
        .route("/nodeinfo/2.1.json", get(nodeinfo::v2_1))
}
