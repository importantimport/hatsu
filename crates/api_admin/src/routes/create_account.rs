use std::ops::Deref;

use activitypub_federation::config::Data;
use axum::{debug_handler, http::StatusCode, Json};
use hatsu_apub::actors::ApubUser;
use hatsu_db_schema::prelude::User;
use hatsu_utils::{AppData, AppError};
use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel};

use crate::entities::{CreateRemoveAccount, CreateRemoveAccountResult};

/// Create Account
#[utoipa::path(
    post,
    tag = "hatsu::admin",
    path = "/api/v0/admin/create-account",
    responses(
        (status = CREATED, description = "create succesfully", body = CreateRemoveAccountResult),
        (status = BAD_REQUEST, description = "error", body = AppError)
    ),
    security(("api_key" = ["token"]))
)]
#[debug_handler]
pub async fn create_account(
    data: Data<AppData>,
    Json(payload): Json<CreateRemoveAccount>,
) -> Result<(StatusCode, Json<CreateRemoveAccountResult>), AppError> {
    if let Some(account) = User::find_by_id(
        hatsu_utils::url::generate_user_url(data.domain(), &payload.name)?.to_string(),
    )
    .one(&data.conn)
    .await?
    {
        Err(AppError::new(
            format!("The account already exists: {}", account.name),
            None,
            Some(StatusCode::BAD_REQUEST),
        ))
    } else {
        let account = ApubUser::new(data.domain(), &payload.name)
            .await?
            .deref()
            .clone()
            .into_active_model()
            .insert(&data.conn)
            .await?;

        Ok((
            StatusCode::CREATED,
            Json(CreateRemoveAccountResult {
                name: account.name.clone(),
                message: format!("The account was successfully created: {}", account.name),
            }),
        ))
    }
}
