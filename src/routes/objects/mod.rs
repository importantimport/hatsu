use axum::{routing::get, Router};
// use axum_extra::routing::RouterExt;

mod notice;
mod object;

pub fn handler() -> Router {
    Router::new()
        // .typed_get(object::handler)
        // .typed_get(object::redirect)
        .route("/notice/*notice", get(notice::redirect))
        .route("/o/*object", get(object::handler))
        .route("/objects/*object", get(object::redirect))
}
