use axum::{
  body::Body,
  Router
};

mod user;
mod well_known;

pub fn init() -> Router<(), Body> {
  let routes = Router::new()
    .merge(user::init())
    .merge(well_known::init());

  routes
}