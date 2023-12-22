use axum::{
    debug_handler,
    extract::Path,
    response::IntoResponse,
    Json,
};

use crate::entities::Context;

// (status = NOT_FOUND, description = "Status is private or does not exist", body = AppError)
// { "error": "Record not found" }

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
pub async fn status_context(
    Path(status_id): Path<String>,
) -> impl IntoResponse {
    Json(Context::find_by_id(status_id))
}