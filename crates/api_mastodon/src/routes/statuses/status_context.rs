use activitypub_federation::config::Data;
use axum::{
    debug_handler,
    extract::Path,
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
    Path(base64_url): Path<String>,
    data: Data<AppData>,
) -> Result<Json<Context>, AppError> {
    let base64 = base64_simd::URL_SAFE;

    match base64.decode_to_vec(&base64_url) {
        Ok(utf8_url) => {
            match String::from_utf8(utf8_url) {
                Ok(url) if url.starts_with("https://") => {
                    let id = format!("https://{}/o/{}", data.domain(), url);
                    let context = Context::find_by_id(id, &data).await?;
                    Ok(Json(context))
                },
                _ => Err(AppError::not_found("Record", &format!("{}", &base64_url)))
            }
        },
        _ => Err(AppError::not_found("Record", &base64_url))
    }
}
