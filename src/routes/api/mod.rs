use axum::Router;

pub fn handler() -> Router {
    Router::new()
        .merge(hatsu_api_admin::routes::handler())
        .merge(hatsu_api_mastodon::routes::handler())
}
