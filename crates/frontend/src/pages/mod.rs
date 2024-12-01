use axum::{routing::get, Router};

mod home;

pub fn routes() -> Router {
    Router::new().route("/", get(home::home))
}
