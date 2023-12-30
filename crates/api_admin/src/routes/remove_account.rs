use activitypub_federation::config::Data;
use axum::{
    debug_handler,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use hatsu_db_schema::prelude::User;
use hatsu_utils::{AppData, AppError};
use sea_orm::*;

use crate::entities::{CreateRemoveAccount, CreateRemoveAccountResult};

/// Remove Account
#[utoipa::path(
    post,
    tag = "hatsu::admin",
    path = "/api/v0/admin/remove-account",
    responses(
        (status = OK, description = "remove succesfully", body = CreateRemoveAccountResult),
        (status = BAD_REQUEST, description = "error", body = AppError)
    ),
    security(("api_key" = ["token"]))
)]
#[debug_handler]
pub async fn remove_account(
    data: Data<AppData>,
    Json(payload): Json<CreateRemoveAccount>,
) -> Result<impl IntoResponse, AppError> {
    match User::find_by_id(format!("https://{}/u/{}", data.domain(), payload.name))
        .one(&data.conn)
        .await? {
            Some(account) => {
                if account.name == data.env.hatsu_primary_account {
                    Err(AppError::new(
                    format!("The primary account for this Hatsu instance could not be removed: {}", account.name), 
                    None,
                    Some(StatusCode::BAD_REQUEST),
                    ))
                } else {
                    // TODO: remove account
                    Ok((StatusCode::OK, Json(CreateRemoveAccountResult {
                        name: payload.name.clone(),
                        message: format!("Successfully removed account: {}", payload.name),
                    })))
                }
            },
            _ => {
                Err(AppError::new(
                format!("The account does not exist: {}", payload.name), 
                None,
                Some(StatusCode::BAD_REQUEST),
                ))
            }
        }
}
