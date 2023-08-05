use axum::{
    body::Body,
    routing::get,
    Router,
};

mod activity;
use activity::activity;

pub fn init() -> Router<(), Body> {
    Router::new()
        .route("/a/:activity", get(activity))
        .route("/activities/:activity", get(activity))
}
