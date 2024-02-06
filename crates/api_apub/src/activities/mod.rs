use axum::{routing::get, Router};
// use axum_extra::routing::RouterExt;

mod activity;

pub fn routes() -> Router {
    Router::new()
        // .typed_get(activity::handler)
        // .typed_get(activity::redirect)
        .route("/a/:activity", get(activity::handler))
        .route("/activities/:activity", get(activity::redirect))
}
