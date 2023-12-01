use activitypub_federation::config::Data;
use axum::{
    debug_handler,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sea_orm::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{
    AppData,
    AppError,
    entities::{
        prelude::*,
        user::Model as DbUser,
    },
};

#[derive(Deserialize, ToSchema)]
pub struct CreateAccount {
    token: Option<String>,
    name: String,
}

#[derive(Serialize, ToSchema)]
pub struct CreateAccountResult {
    name: String,
    message: String,
}

/// Create Account
#[utoipa::path(
    post,
    tag = "hatsu::admin",
    path = "/api/hatsu/v0/admin/create-account",
    responses(
        (status = 201, description = "create succesfully", body = CreateAccountResult),
        (status = NOT_FOUND, description = "error", body = AppError)
    ),
    params(
        ("name" = String, Query, description = "username"),
    ),
    // security(("api_key" = ["token"]))
)]
#[debug_handler]
pub async fn create_account(
    data: Data<AppData>,
    Json(payload): Json<CreateAccount>,
) -> Result<impl IntoResponse, AppError> {
    match payload.token {
        token if (token.is_some() && token == data.env.hatsu_access_token) => {
            match User::find_by_id(format!("https://{}/u/{}", data.domain(), payload.name))
                .one(&data.conn)
                .await? {
                    Some(account) => Err(AppError::new(
                        format!("The account already exists: {}", account.name), 
                        None,
                        Some(StatusCode::BAD_REQUEST),
                    )),
                    _ => {
                        let account = DbUser::new(data.domain(), &payload.name).await?;
                        let account = account.into_active_model().insert(&data.conn).await?;
                        Ok((StatusCode::CREATED, Json(CreateAccountResult {
                            name: account.name.clone(),
                            message: format!("The account was successfully created: {}", account.name),
                        })))
                    }
                }
        }
        _ => Err(AppError::new(
            "Access Token Authentication Failed".to_string(), 
            None,
            Some(StatusCode::FORBIDDEN),
        ))
    }
}
