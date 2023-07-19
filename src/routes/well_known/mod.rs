use axum::{
  body::Body,
  routing::get,
  Router,
};

mod nodeinfo;
pub use nodeinfo::nodeinfo;

mod webfinger;
pub use webfinger::webfinger;

pub fn init() -> Router<(), Body> {
  let well_known = Router::new()
    .route("/.well-known/nodeinfo", get(nodeinfo))
    .route("/.well-known/webfinger", get(webfinger));

  well_known
}