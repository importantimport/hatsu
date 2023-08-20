use axum::{
    body::Body,
    routing::get,
    Router,
};

mod object;

pub fn init() -> Router<(), Body> {
    Router::new()
        .route("/o/*object", get(object::handler))
        .route("/objects/*object", get(object::redirect))
}
