use axum::{
    routing::post,
    Router,
};

pub mod create_account;
use create_account::create_account;

pub mod remove_account;
use remove_account::remove_account;

pub fn handler() -> Router {
    Router::new()
        .route("/api/hatsu/v0/admin/create-account", post(create_account))
        .route("/api/hatsu/v0/admin/remove-account", post(remove_account))
}
