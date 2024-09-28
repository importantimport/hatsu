use axum::routing::{get, post};
use serde::Deserialize;
use utoipa::{IntoParams, OpenApi};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::ApubApi;
pub mod user;
mod user_followers;
mod user_following;
mod user_inbox;
mod user_outbox;

#[derive(Deserialize, IntoParams)]
pub struct Pagination {
    page: Option<u64>,
}

pub fn routes() -> OpenApiRouter {
    OpenApiRouter::with_openapi(ApubApi::openapi())
        .routes(routes!(user::handler))
        .routes(routes!(user_followers::handler))
        .routes(routes!(user_following::handler))
        .routes(routes!(user_inbox::handler))
        .routes(routes!(user_outbox::handler))
        // fallback routes
        .route("/u/:user", get(user::redirect))
        .route("/u/:user/followers", get(user_followers::redirect))
        .route("/u/:user/following", get(user_following::redirect))
        .route("/u/:user/outbox", get(user_outbox::redirect))
        .route("/u/:user/inbox", post(user_inbox::handler))
}
