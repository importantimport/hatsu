use axum::{
  body::Body,
  routing::{
    get,
    post,
  },
  Router,
};

mod user;
pub use user::user;

mod inbox;
pub use inbox::user_inbox;

pub fn init() -> Router<(), Body> {
  let router = Router::new()
    .route("/:user", get(user))
    .route("/:user/inbox", post(user_inbox));

  router
}