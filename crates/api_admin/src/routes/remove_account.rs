use activitypub_federation::config::Data;
use axum::{debug_handler, http::StatusCode, Json};
use hatsu_db_schema::prelude::User;
use hatsu_utils::{AppData, AppError};
use sea_orm::EntityTrait;

use crate::entities::{CreateRemoveAccount, CreateRemoveAccountResult};

/// Remove Account
#[utoipa::path(
    post,
    tag = "hatsu::admin",
    path = "/api/v0/admin/remove-account",
    responses(
        // (status = OK, description = "remove succesfully", body = CreateRemoveAccountResult),
        (status = METHOD_NOT_ALLOWED, description = "not implemented", body = CreateRemoveAccountResult),
        (status = BAD_REQUEST, description = "error", body = AppError)
    ),
    security(("api_key" = ["token"]))
)]
#[debug_handler]
pub async fn remove_account(
    data: Data<AppData>,
    Json(payload): Json<CreateRemoveAccount>,
) -> Result<(StatusCode, Json<CreateRemoveAccountResult>), AppError> {
    match User::find_by_id(
        hatsu_utils::url::generate_user_url(data.domain(), &payload.name)?.to_string(),
    )
    .one(&data.conn)
    .await?
    {
        Some(account) => {
            if account.name == data.env.hatsu_primary_account {
                Err(AppError::new(
                    format!(
                        "The primary account for this Hatsu instance could not be removed: {}",
                        account.name
                    ),
                    None,
                    Some(StatusCode::BAD_REQUEST),
                ))
            } else {
                // TODO: remove account
                Ok((
                    StatusCode::METHOD_NOT_ALLOWED,
                    Json(CreateRemoveAccountResult {
                        name: payload.name.clone(),
                        // message: format!("Successfully removed account: {}", payload.name),
                        message: format!(
                            "Remove account API not yet implemented: {}",
                            payload.name
                        ),
                    }),
                ))
            }
        },
        None => Err(AppError::new(
            format!("The account does not exist: {}", payload.name),
            None,
            Some(StatusCode::BAD_REQUEST),
        )),
    }
}
