use activitypub_federation::config::Data;
use anyhow::anyhow;
use axum::{
    debug_handler,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use sea_orm::*;
use serde::{Deserialize, Serialize};

use crate::{
    AppData,
    AppError,
    entities::prelude::*,
};

#[derive(Deserialize)]
pub struct RemoveAccount {
    token: Option<String>,
    name: String,
}

#[derive(Serialize)]
pub struct RemoveAccountResult {
    name: String,
    message: String,
}

#[debug_handler]
pub async fn remove_account(
    data: Data<AppData>,
    Json(payload): Json<RemoveAccount>,
) -> Result<impl IntoResponse, AppError> {
    match payload.token {
        token if (token.is_some() && token == data.env.hatsu_access_token) => {
            match User::find_by_id(format!("https://{}/u/{}", data.domain(), payload.name))
                .one(&data.conn)
                .await? {
                    Some(account) => {
                        if account.name == data.env.hatsu_primary_account {
                            Ok((StatusCode::BAD_REQUEST, Json(RemoveAccountResult {
                                name: account.name.clone(),
                                message: format!("The primary account for this Hatsu instance could not be removed: {}", account.name)
                            })))
                        } else {
                            // TODO: remove account
                            Ok((StatusCode::OK, Json(RemoveAccountResult {
                                name: payload.name.clone(),
                                message: format!("Successfully removed account: {}", payload.name),
                            })))
                        }
                    },
                    _ => {
                        Ok((StatusCode::BAD_REQUEST, Json(RemoveAccountResult {
                            name: payload.name.clone(),
                            message: format!("The account does not exist: {}", payload.name)
                        })))
                    }
                }
        }
        // TODO: StatusCode::FORBIDDEN
        _ => Err(anyhow!("Access Token Authentication Failed").into())
    }
}
