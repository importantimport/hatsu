use activitypub_federation::config::Data;
use axum::{
    http::{Request, StatusCode},
    middleware::{self, Next},
    response::Response,
    routing::post,
    Router,
};
use hatsu_utils::AppData;

pub mod create_account;
pub mod remove_account;

use create_account::create_account;
use remove_account::remove_account;

pub fn routes() -> Router {
    Router::new()
        .route("/api/v0/admin/create-account", post(create_account))
        .route("/api/v0/admin/remove-account", post(remove_account))
        .layer(middleware::from_fn(auth))
}

async fn auth<B>(
    data: Data<AppData>,
    request: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    match &data.env.hatsu_access_token {
        Some(token) => match request.uri().query() {
            Some(query) if query == format!("token={token}") => Ok(next.run(request).await),
            Some(query) if query != format!("token={token}") => Err(StatusCode::UNAUTHORIZED),
            _ => Err(StatusCode::BAD_REQUEST),
        },
        None => Err(StatusCode::UNAUTHORIZED),
    }
}
