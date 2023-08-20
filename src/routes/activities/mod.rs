use axum::{
    body::Body,
    routing::get,
    Router,
};

mod activity;

pub fn init() -> Router<(), Body> {
    Router::new()
        .route("/a/:activity", get(activity::handler))
        .route("/activities/:activity", get(activity::redirect))
}
