use axum::{
    body::Body,
    routing::get,
    Router,
};

mod object;
use object::object;

pub fn init() -> Router<(), Body> {
    Router::new()
        .route("/o/*object", get(object))
        .route("/objects/*object", get(object))
}
