use axum::{routing::get, Router};
// use axum_extra::routing::RouterExt;

mod object;

pub fn handler() -> Router {
    Router::new()
        // .typed_get(object::handler)
        // .typed_get(object::redirect)
        .route("/o/*object", get(object::handler))
        .route("/objects/*object", get(object::redirect))
}
