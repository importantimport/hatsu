use axum::{
    routing::{get, post},
    Router,
};
// use axum_extra::routing::RouterExt;

pub mod user;
mod user_followers;
mod user_following;
mod user_inbox;
mod user_outbox;

pub fn routes() -> Router {
    Router::new()
        // .typed_get(user::handler)
        // .typed_get(user::redirect)
        // .typed_get(user_followers::handler)
        // .typed_get(user_followers::redirect)
        // .typed_get(user_outbox::handler)
        // .typed_get(user_outbox::redirect)
        .route("/u/:user", get(user::redirect))
        .route("/u/:user/followers", get(user_followers::redirect))
        .route("/u/:user/following", get(user_following::redirect))
        .route("/u/:user/outbox", get(user_outbox::redirect))
        .route("/u/:user/inbox", post(user_inbox::handler))
        .route("/users/:user", get(user::user))
        .route("/users/:user/followers", get(user_followers::handler))
        .route("/users/:user/following", get(user_following::handler))
        .route("/users/:user/outbox", get(user_outbox::handler))
        .route("/users/:user/inbox", post(user_inbox::handler))
}
