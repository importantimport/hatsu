use axum::{
    body::Body,
    routing::{get, post},
    Router,
};

mod user;
pub use user::user;

mod inbox;
pub use inbox::user_inbox;

pub fn init() -> Router<(), Body> {
    Router::new()
        .route("/u/:user", get(user))
        .route("/users/:user", get(user))
        .route("/u/:user/inbox", post(user_inbox))
        .route("/users/:user/inbox", post(user_inbox))
}