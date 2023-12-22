use axum::{
    debug_handler,
    response::IntoResponse,
    Json,
};

use crate::entities::Account;

// (status = NOT_FOUND, description = "Status does not exist or is private", body = AppError)
// { "error": "Record not found" }

/// See who favourited a status
///
/// https://docs.joinmastodon.org/methods/statuses/#favourited_by
#[utoipa::path(
    post,
    tag = "mastodon",
    path = "/api/v1/statuses/{id}/favourited_by",
    responses(
        (status = OK, description = "A list of accounts who favourited the status", body = Vec<Account>),
    ),
    params(
        ("id" = String, Path, description = "The ID of the Status in the database.")
    )
)]
#[debug_handler]
pub async fn status_favourited_by() -> impl IntoResponse {
    Json(vec![Account {}])
}
