use axum::{
    body::Body,
    Router,
};
use axum_extra::routing::RouterExt;

mod activity;

pub fn init() -> Router<(), Body> {
    Router::new()
        .typed_get(activity::handler)
        .typed_get(activity::redirect)
}
