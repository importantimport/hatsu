use activitypub_federation::config::Data;
use axum::{debug_handler, extract::Query, http::StatusCode, Json};
use hatsu_db_schema::{blocked_url, prelude::BlockedUrl};
use hatsu_utils::{AppData, AppError};
use sea_orm::{ActiveModelTrait, EntityTrait, Set};

use crate::{
    entities::{BlockUrlQuery, BlockUrlResult},
    TAG,
};

/// Remove Account
#[utoipa::path(
    post,
    tag = TAG,
    path = "/api/v0/admin/block-url",
    params(BlockUrlQuery),
    responses(
        (status = OK, description = "block successfully", body = BlockUrlResult),
        (status = BAD_REQUEST, description = "error", body = AppError)
    ),
    security(("api_key" = ["token"]))
)]
#[debug_handler]
pub async fn block_url(
    data: Data<AppData>,
    query: Query<BlockUrlQuery>,
) -> Result<(StatusCode, Json<BlockUrlResult>), AppError> {
    match BlockedUrl::find_by_id(&query.url.to_string())
        .one(&data.conn)
        .await?
    {
        Some(url) => Err(AppError::new(
            format!("The url already blocked: {}", url.id),
            None,
            Some(StatusCode::BAD_REQUEST),
        )),
        None => {
            blocked_url::ActiveModel {
                id: Set(query.url.to_string()),
                is_instance: Set(query.url.path().eq("/")),
            }
            .insert(&data.conn)
            .await?;

            Ok((
                StatusCode::CREATED,
                Json(BlockUrlResult {
                    url: query.url.clone(),
                    message: format!(
                        "The url was successfully blocked: {}",
                        &query.url.to_string()
                    ),
                }),
            ))
        },
    }
}
