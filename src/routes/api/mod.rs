use axum::Router;

pub mod v0;
pub mod v1;

pub fn handler() -> Router {
    Router::new()
        .merge(v0::handler())
        .merge(v1::handler())
}
