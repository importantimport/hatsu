use axum::{response::Response, routing::get, Router};
use hatsu_utils::AppEnv;

// ./hatsu --version
async fn root() -> Response<String> {
    Response::new(AppEnv::info())
}

#[must_use]
pub fn routes() -> Router {
    Router::new().route("/", get(root))
}
