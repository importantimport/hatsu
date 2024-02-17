use axum::{routing::get, Router};
// use axum_extra::routing::RouterExt;

mod notice;
pub mod post;

pub fn routes() -> Router {
    Router::new()
        // .typed_get(object::handler)
        // .typed_get(object::redirect)
        .route("/notice/*notice", get(notice::notice))
        .route("/posts/*post", get(post::post))
        .route("/p/*post", get(post::redirect))
}
