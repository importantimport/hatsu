use axum::{routing::get, Router};

pub mod generate_204;

use generate_204::generate_204;

pub fn routes() -> Router {
    Router::new().route("/api/v0/generate_204", get(generate_204))
}
