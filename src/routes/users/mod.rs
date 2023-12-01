use axum::{
    routing::post,
    Router,
};
use axum_extra::routing::RouterExt;

mod user;
mod user_followers;
mod user_inbox;
mod user_outbox;

pub fn handler() -> Router {
    Router::new()
        .typed_get(user::handler)
        .typed_get(user::redirect)
        .typed_get(user_followers::handler)
        .typed_get(user_followers::redirect)
        .typed_get(user_outbox::handler)
        .typed_get(user_outbox::redirect)
        .route("/u/:user/inbox", post(user_inbox::handler))
        .route("/users/:user/inbox", post(user_inbox::handler))
}
