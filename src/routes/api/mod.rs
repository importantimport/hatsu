use axum::Router;

pub mod hatsu;
pub mod v1;

pub fn handler() -> Router {
    Router::new()
        .merge(hatsu::handler())
        .merge(v1::handler())
}
