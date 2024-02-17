use axum::{routing::get, Router};
// use axum_extra::routing::RouterExt;

pub mod activity;

pub fn routes() -> Router {
    Router::new()
        // .typed_get(activity::handler)
        // .typed_get(activity::redirect)
        .route("/activities/:activity", get(activity::activity))
        .route("/a/:activity", get(activity::redirect))
}
