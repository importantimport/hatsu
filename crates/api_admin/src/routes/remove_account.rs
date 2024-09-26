use activitypub_federation::config::Data;
use axum::{debug_handler, extract::Query, http::StatusCode, Json};
use hatsu_db_schema::prelude::User;
use hatsu_utils::{AppData, AppError};
use sea_orm::EntityTrait;

use crate::{
    entities::{CreateRemoveAccountQuery, CreateRemoveAccountResult},
    TAG,
};

/// Remove Account
#[utoipa::path(
    post,
    tag = TAG,
    path = "/api/v0/admin/remove-account",
    params(CreateRemoveAccountQuery),
    responses(
        // (status = OK, description = "remove successfully", body = CreateRemoveAccountResult),
        (status = METHOD_NOT_ALLOWED, description = "not implemented", body = CreateRemoveAccountResult),
        (status = BAD_REQUEST, description = "error", body = AppError)
    ),
    security(("api_key" = ["token"]))
)]
#[debug_handler]
pub async fn remove_account(
    data: Data<AppData>,
    query: Query<CreateRemoveAccountQuery>,
) -> Result<(StatusCode, Json<CreateRemoveAccountResult>), AppError> {
    match User::find_by_id(
        hatsu_utils::url::generate_user_url(data.domain(), &query.name)?.to_string(),
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
                        name: query.name.clone(),
                        // message: format!("Successfully removed account: {}", payload.name),
                        message: format!("Remove account API not yet implemented: {}", query.name),
                    }),
                ))
            }
        },
        None => Err(AppError::new(
            format!("The account does not exist: {}", query.name),
            None,
            Some(StatusCode::BAD_REQUEST),
        )),
    }
}
