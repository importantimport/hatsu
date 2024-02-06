use axum::{routing::get, Router};

mod host_meta;
mod nodeinfo;
mod webfinger;

pub use host_meta::{host_meta, host_meta_json, host_meta_xml};
pub use nodeinfo::nodeinfo;
pub use webfinger::webfinger;

pub fn routes() -> Router {
    Router::new()
        .route("/.well-known/host-meta", get(host_meta))
        .route("/.well-known/host-meta.xml", get(host_meta_xml))
        .route("/.well-known/host-meta.json", get(host_meta_json))
        .route("/.well-known/nodeinfo", get(nodeinfo))
        .route("/.well-known/webfinger", get(webfinger))
}
