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
    AppError,
    entities::{
        prelude::*,
        user::Model as DbUser,
    },
};

// TODO: Serialize => JsonSchema
#[derive(Deserialize, Serialize)]
pub struct CreateAccount {
    token: Option<String>,
    name: String,
}

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
                    Some(account) => Ok((StatusCode::BAD_REQUEST, Json(account.into_json(&data).await?))),
                    _ => {
                        let account = DbUser::new(&payload.name).await?;
                        let account = account.into_active_model().insert(&data.conn).await?;
                        Ok((StatusCode::CREATED, Json(account.into_json(&data).await?)))
                    }
                }
        }
        // TODO: StatusCode::FORBIDDEN
        _ => Err(anyhow!("Access Token Authentication Failed").into())
    }
}
