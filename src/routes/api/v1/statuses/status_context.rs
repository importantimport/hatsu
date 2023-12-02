use axum::{
    debug_handler,
    response::IntoResponse,
    Json,
};
use serde::Serialize;

/// https://docs.joinmastodon.org/entities/Context/
/// https://docs.joinmastodon.org/methods/statuses/#context
#[derive(Debug, Serialize)]
pub struct Context {
    // TODO: Vec<Status>
    // should always be empty vec
    ancestors: Vec<String>,
    // TODO: Vec<Status>
    descendants: Vec<String>,
}

#[debug_handler]
pub async fn status_context() -> impl IntoResponse {
    Json(Context {
        ancestors: vec![],
        descendants: vec![]
    })
}
