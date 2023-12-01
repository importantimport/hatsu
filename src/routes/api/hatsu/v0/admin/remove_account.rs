use activitypub_federation::config::Data;
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
                            Err(AppError::new(
                                format!("The primary account for this Hatsu instance could not be removed: {}", account.name), 
                                None,
                                Some(StatusCode::BAD_REQUEST),
                            ))
                        } else {
                            // TODO: remove account
                            Ok((StatusCode::OK, Json(RemoveAccountResult {
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
        _ => Err(AppError::new(
            "Access Token Authentication Failed".to_string(), 
            None,
            Some(StatusCode::FORBIDDEN),
        ))
    }
}
