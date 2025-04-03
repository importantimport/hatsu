use axum::{Router, routing::get};

mod home;

pub fn routes() -> Router {
    Router::new().route("/", get(home::home))
}
