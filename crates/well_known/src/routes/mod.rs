use axum::{routing::get, Router};

pub mod host_meta;
pub mod nodeinfo;
pub mod webfinger;

pub fn routes() -> Router {
    Router::new()
        .route("/.well-known/host-meta", get(host_meta::redirect))
        .route("/.well-known/host-meta.xml", get(host_meta::xml))
        .route("/.well-known/host-meta.json", get(host_meta::json))
        .route("/.well-known/nodeinfo", get(nodeinfo::discovery))
        .route("/.well-known/webfinger", get(webfinger::webfinger))
}
