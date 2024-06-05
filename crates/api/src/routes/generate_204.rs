use axum::{debug_handler, http::StatusCode};

/// Generate 204 Response
#[utoipa::path(
    get,
    tag = "hatsu",
    path = "/api/v0/generate_204",
    responses(
        (status = NO_CONTENT, description = "NO_CONTENT"),
    )
)]
#[debug_handler]
pub async fn generate_204() -> StatusCode {
    StatusCode::NO_CONTENT
}
