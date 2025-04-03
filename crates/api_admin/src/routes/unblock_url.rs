use activitypub_federation::config::Data;
use axum::{Json, debug_handler, extract::Query, http::StatusCode};
use hatsu_db_schema::prelude::BlockedUrl;
use hatsu_utils::{AppData, AppError};
use sea_orm::{EntityTrait, ModelTrait};

use crate::{
    TAG,
    entities::{BlockUrlQuery, BlockUrlResult},
};

/// Unblock URL
#[utoipa::path(
    post,
    tag = TAG,
    path = "/api/v0/admin/unblock-url",
    params(BlockUrlQuery),
    responses(
        (status = OK, description = "unblock successfully", body = BlockUrlResult),
        (status = BAD_REQUEST, description = "error", body = AppError)
    ),
    security(("api_key" = ["token"]))
)]
#[debug_handler]
pub async fn unblock_url(
    data: Data<AppData>,
    query: Query<BlockUrlQuery>,
) -> Result<(StatusCode, Json<BlockUrlResult>), AppError> {
    match BlockedUrl::find_by_id(query.url.to_string())
        .one(&data.conn)
        .await?
    {
        Some(url) => {
            url.delete(&data.conn).await?;

            Ok((
                StatusCode::OK,
                Json(BlockUrlResult {
                    url: query.url.clone(),
                    message: format!("The url was successfully unblocked: {}", &query.url),
                }),
            ))
        },
        None => Err(AppError::new(
            format!("The url doesn't exist: {}", query.url),
            None,
            Some(StatusCode::BAD_REQUEST),
        )),
    }
}
