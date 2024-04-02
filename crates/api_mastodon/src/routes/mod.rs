use axum::Router;

pub mod instance;
pub mod statuses;

pub fn routes() -> Router {
    Router::new()
        .merge(instance::routes())
        .merge(statuses::routes())
}
