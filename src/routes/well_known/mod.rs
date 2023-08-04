use axum::{
    body::Body,
    routing::get,
    Router,
};

mod host_meta;
pub use host_meta::{host_meta, host_meta_json};

mod nodeinfo;
pub use nodeinfo::nodeinfo;

mod webfinger;
pub use webfinger::webfinger;

pub fn init() -> Router<(), Body> {
    Router::new()
        .route("/.well-known/host-meta", get(host_meta))
        .route("/.well-known/host-meta.json", get(host_meta_json))
        .route("/.well-known/nodeinfo", get(nodeinfo))
        .route("/.well-known/webfinger", get(webfinger))
}