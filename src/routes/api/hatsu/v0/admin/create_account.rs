use activitypub_federation::{
    config::Data,
    traits::Object,
};
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
    entities::{
        prelude::*,
        user::Model as DbUser,
    }, error::AppError
};

#[derive(Deserialize, Serialize)]
pub struct CreateAccount {
    access_token: Option<String>,
    name: String,
}

#[debug_handler]
pub async fn create_account(
    data: Data<AppData>,
    Json(payload): Json<CreateAccount>,
) -> Result<impl IntoResponse, AppError> {
    match payload.access_token {
        access_token if (
            access_token.is_some() &&
            access_token == data.env.hatsu_access_token
        ) => {
            match User::find_by_id(format!("https://{}/u/{}", data.domain(), payload.name))
                .one(&data.conn)
                .await? {
                    Some(account) => Ok((StatusCode::BAD_REQUEST, Json(account.into_json(&data).await?))),
                    _ => {
                        let account = DbUser::new(&payload.name).await?;
                        let account = account.into_active_model().insert(&data.conn).await?;
                        Ok((StatusCode::CREATED, Json(account.into_json(&data).await?)))
                    }
                }
        }
        // TODO: StatusCode::FORBIDDEN
        _ => Err(AppError::Anyhow(anyhow!("Access Token Authentication Failed")))
    }
}
