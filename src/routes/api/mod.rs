use axum::Router;

pub mod v0;

pub fn handler() -> Router {
    Router::new()
        .merge(v0::handler())
        .merge(hatsu_api_mastodon::routes::handler())
}
