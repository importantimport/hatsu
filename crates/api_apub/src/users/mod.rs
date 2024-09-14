use axum::routing::{get, post};
use utoipa::OpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::ApubApi;
pub mod user;
mod user_followers;
mod user_following;
mod user_inbox;
mod user_outbox;

pub fn routes() -> OpenApiRouter {
    OpenApiRouter::with_openapi(ApubApi::openapi())
        .routes(routes!(user::handler))
        // TODO: writing utoipa docs
        .route("/users/:user/followers", get(user_followers::handler))
        .route("/users/:user/following", get(user_following::handler))
        .route("/users/:user/outbox", get(user_outbox::handler))
        .route("/users/:user/inbox", post(user_inbox::handler))
        // fallback routes
        .route("/u/:user", get(user::redirect))
        .route("/u/:user/followers", get(user_followers::redirect))
        .route("/u/:user/following", get(user_following::redirect))
        .route("/u/:user/outbox", get(user_outbox::redirect))
        .route("/u/:user/inbox", post(user_inbox::handler))
}
