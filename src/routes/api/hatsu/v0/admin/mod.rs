use axum::{
    body::Body,
    routing::post,
    Router,
};

mod create_account;
use create_account::create_account;

pub fn init() -> Router<(), Body> {
    Router::new()
        .route("/api/hatsu/v0/admin/create-account", post(create_account))
}
