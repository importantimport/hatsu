use axum::{
    routing::get,
    Router,
};

mod host_meta;
pub use host_meta::{host_meta, host_meta_xrd, host_meta_json};

mod nodeinfo;
pub use nodeinfo::nodeinfo;

mod webfinger;
pub use webfinger::webfinger;

pub fn handler() -> Router {
    Router::new()
        .route("/.well-known/host-meta", get(host_meta))
        .route("/.well-known/host-meta.xrd", get(host_meta_xrd))
        .route("/.well-known/host-meta.xml", get(host_meta_xrd))
        .route("/.well-known/host-meta.jrd", get(host_meta_json))
        .route("/.well-known/host-meta.json", get(host_meta_json))
        .route("/.well-known/nodeinfo", get(nodeinfo))
        .route("/.well-known/webfinger", get(webfinger))
}
