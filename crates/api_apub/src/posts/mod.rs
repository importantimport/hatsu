use axum::{routing::get, Router};
// use axum_extra::routing::RouterExt;

mod notice;
mod post;

pub fn routes() -> Router {
    Router::new()
        // .typed_get(object::handler)
        // .typed_get(object::redirect)
        .route("/notice/*notice", get(notice::notice))
        .route("/o/*object", get(post::handler))
        .route("/objects/*object", get(post::redirect))
}
