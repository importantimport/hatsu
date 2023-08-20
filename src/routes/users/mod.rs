use axum::{
    body::Body,
    routing::{get, post},
    Router,
};

mod user;
mod user_inbox;
mod user_outbox;

pub fn init() -> Router<(), Body> {
    Router::new()
        .route("/u/:user", get(user::handler))
        .route("/users/:user", get(user::redirect))
        .route("/u/:user/inbox", post(user_inbox::handler))
        .route("/users/:user/inbox", post(user_inbox::handler))
        .route("/u/:user/outbox", get(user_outbox::handler))
        .route("/users/:user/outbox", get(user_outbox::redirect))
}