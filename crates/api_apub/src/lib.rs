use axum::Router;

pub mod activities;
pub mod posts;
pub mod users;

pub fn routes() -> Router {
    Router::new()
        .merge(activities::routes())
        .merge(posts::routes())
        .merge(users::routes())
}
