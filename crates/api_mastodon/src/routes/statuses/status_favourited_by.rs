use activitypub_federation::config::Data;
use axum::{debug_handler, Json};
use hatsu_utils::{AppData, AppError};

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
pub async fn status_favourited_by(data: Data<AppData>) -> Result<Json<Vec<Account>>, AppError> {
    Ok(Json(vec![Account::primary_account(&data).await?]))
}
