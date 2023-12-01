use axum::{
    body::Body,
    Router,
};
use axum_extra::routing::RouterExt;

mod object;

pub fn init() -> Router<(), Body> {
    Router::new()
        .typed_get(object::handler)
        .typed_get(object::redirect)
}
