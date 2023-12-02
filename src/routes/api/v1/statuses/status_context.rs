use axum::{
    debug_handler,
    response::IntoResponse,
    Json,
};
use serde::Serialize;
use utoipa::ToSchema;

/// https://docs.joinmastodon.org/entities/Context/
/// https://docs.joinmastodon.org/methods/statuses/#context
#[derive(Debug, Serialize, ToSchema)]
pub struct Context {
    // TODO: Vec<Status>
    // should always be empty vec
    ancestors: Vec<String>,
    // TODO: Vec<Status>
    descendants: Vec<String>,
}

/// Get parent and child statuses in context
///
/// https://docs.joinmastodon.org/methods/statuses/#context
#[utoipa::path(
    post,
    tag = "mastodon",
    path = "/api/v1/statuses/{id}/context",
    responses(
        (status = OK, description = "", body = Context),
    ),
    params(
        ("id" = String, Path, description = "The ID of the Status in the database.")
    )
)]
#[debug_handler]
pub async fn status_context() -> impl IntoResponse {
    Json(Context {
        ancestors: vec![],
        descendants: vec![]
    })
}
