use activitypub_federation::config::Data;
use axum::{debug_handler, extract::Path, Json};
use hatsu_db_schema::prelude::{Post, ReceivedLike};
use hatsu_utils::{AppData, AppError};
use sea_orm::*;

use crate::entities::Account;

// (status = NOT_FOUND, description = "Status does not exist or is private", body = AppError)
// { "error": "Record not found" }

/// See who favourited a status
///
/// <https://docs.joinmastodon.org/methods/statuses/#favourited_by>
#[utoipa::path(
    get,
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
pub async fn status_favourited_by(
    Path(base64_url): Path<String>,
    data: Data<AppData>,
) -> Result<Json<Vec<Account>>, AppError> {
    // Ok(Json(vec![Account::primary_account(&data).await?]))
    let base64 = base64_simd::URL_SAFE;

    match base64.decode_to_vec(&base64_url) {
        Ok(utf8_url) => match String::from_utf8(utf8_url) {
            Ok(url) if url.starts_with("https://") => {
                let post_url = hatsu_utils::url::generate_post_url(data.domain(), url)?;

                match Post::find_by_id(&post_url.to_string())
                    .one(&data.conn)
                    .await?
                {
                    Some(post) => {
                        let handles = post
                            .find_related(ReceivedLike)
                            .all(&data.conn)
                            .await
                            .unwrap()
                            .into_iter()
                            .map(|received_like| async {
                                Account::from_id(received_like.actor, &data).await.unwrap()
                            })
                            .collect::<Vec<_>>();

                        Ok(Json(futures::future::join_all(handles).await))
                    },
                    _ => Err(AppError::not_found("Record", &base64_url)),
                }
            },
            _ => Err(AppError::not_found("Record", &base64_url)),
        },
        _ => Err(AppError::not_found("Record", &base64_url)),
    }
}
