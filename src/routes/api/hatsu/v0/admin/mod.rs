use std::env;

use axum::{
    http::{Request, StatusCode},
    middleware::{self, Next},
    response::Response,
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
        .layer(middleware::from_fn(auth))
}

async fn auth<B>(request: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
    // TODO: no longer use std::env
    match env::var("HATSU_ACCESS_TOKEN").unwrap_or("".to_string()) {
        token if token != "".to_string() => {
            match request.uri().query() {
                Some(query) if query == format!("token={}", token) => Ok(next.run(request).await),
                Some(query) if query != format!("token={}", token) => Err(StatusCode::UNAUTHORIZED),
                _ => Err(StatusCode::BAD_REQUEST)
            }
        },
        _ => Err(StatusCode::UNAUTHORIZED)
    }
}
