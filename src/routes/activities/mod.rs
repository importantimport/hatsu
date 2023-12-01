use axum::Router;
use axum_extra::routing::RouterExt;

mod activity;

pub fn handler() -> Router {
    Router::new()
        .typed_get(activity::handler)
        .typed_get(activity::redirect)
}
