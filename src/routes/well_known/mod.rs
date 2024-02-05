use axum::{routing::get, Router};

mod host_meta;
pub use host_meta::{host_meta, host_meta_json, host_meta_xml};

mod webfinger;
pub use webfinger::webfinger;

pub fn handler() -> Router {
    Router::new()
        .route("/.well-known/host-meta", get(host_meta))
        .route("/.well-known/host-meta.xml", get(host_meta_xml))
        .route("/.well-known/host-meta.json", get(host_meta_json))
        .route("/.well-known/webfinger", get(webfinger))
}
