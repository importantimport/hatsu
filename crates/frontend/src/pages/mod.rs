use axum::{routing::get, Router};

mod home;

#[must_use]
pub fn routes() -> Router {
    Router::new().route("/", get(home::home))
}
