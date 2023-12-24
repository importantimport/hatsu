use activitypub_federation::config::Data;
use axum::{
    debug_handler,
    extract::Path,
    response::IntoResponse,
    Json,
};
use hatsu_utils::{AppData, AppError};

use crate::entities::Context;

/// Get parent and child statuses in context
///
/// https://docs.joinmastodon.org/methods/statuses/#context
#[utoipa::path(
    post,
    tag = "mastodon",
    path = "/api/v1/statuses/{id}/context",
    responses(
        (status = OK, description = "", body = Context),
        (status = NOT_FOUND, description = "Status is private or does not exist", body = AppError)
    ),
    params(
        ("id" = String, Path, description = "The ID of the Status in the database.")
    )
)]
#[debug_handler]
pub async fn status_context(
    Path(status_id): Path<String>,
    data: Data<AppData>,
) -> Result<impl IntoResponse, AppError> {
    let context = Context::find_by_id(status_id, &data).await?;
    Ok(Json(context))
}
