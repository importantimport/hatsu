use std::ops::Deref;

use activitypub_federation::config::Data;
use axum::{Json, debug_handler, extract::Query, http::StatusCode};
use hatsu_apub::actors::ApubUser;
use hatsu_db_schema::prelude::User;
use hatsu_utils::{AppData, AppError};
use sea_orm::{ActiveModelTrait, EntityTrait, IntoActiveModel};

use crate::{
    TAG,
    entities::{CreateRemoveAccountQuery, CreateRemoveAccountResult},
};

/// Create Account
#[utoipa::path(
    post,
    tag = TAG,
    path = "/api/v0/admin/create-account",
    params(CreateRemoveAccountQuery),
    responses(
        (status = CREATED, description = "create successfully", body = CreateRemoveAccountResult),
        (status = BAD_REQUEST, description = "error", body = AppError)
    ),
    security(("api_key" = ["token"]))
)]
#[debug_handler]
pub async fn create_account(
    data: Data<AppData>,
    query: Query<CreateRemoveAccountQuery>,
) -> Result<(StatusCode, Json<CreateRemoveAccountResult>), AppError> {
    if let Some(account) = User::find_by_id(
        hatsu_utils::url::generate_user_url(data.domain(), &query.name)?.to_string(),
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
        let account = ApubUser::new(data.domain(), &query.name)
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
