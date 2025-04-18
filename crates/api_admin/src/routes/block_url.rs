use activitypub_federation::config::Data;
use axum::{Json, debug_handler, extract::Query, http::StatusCode};
use hatsu_db_schema::{blocked_url, prelude::BlockedUrl};
use hatsu_utils::{AppData, AppError};
use sea_orm::{ActiveModelTrait, EntityTrait, Set};

use crate::{
    TAG,
    entities::{BlockUrlQuery, BlockUrlResult},
};

/// Block URL
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
    match &query.url {
        url if url.query().is_some() => Err(AppError::new(
            format!("wrong url: {url} (can't contain search params)"),
            None,
            Some(StatusCode::BAD_REQUEST),
        )),
        _ =>
            if let Some(url) = BlockedUrl::find_by_id(query.url.to_string())
                .one(&data.conn)
                .await?
            {
                Err(AppError::new(
                    format!("The url already blocked: {}", url.id),
                    None,
                    Some(StatusCode::BAD_REQUEST),
                ))
            } else {
                blocked_url::ActiveModel {
                    id: Set(query.url.to_string()),
                    is_instance: Set(query.url.path().eq("/")),
                }
                .insert(&data.conn)
                .await?;

                Ok((
                    StatusCode::OK,
                    Json(BlockUrlResult {
                        url: query.url.clone(),
                        message: format!("The url was successfully blocked: {}", &query.url),
                    }),
                ))
            },
    }
}
