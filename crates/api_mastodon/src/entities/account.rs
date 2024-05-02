use activitypub_federation::{config::Data, traits::Object};
use hatsu_apub::actors::{ApubUser, Service};
use hatsu_db_schema::prelude::User;
use hatsu_utils::{AppData, AppError};
use sea_orm::EntityTrait;
use serde::{Deserialize, Serialize};
use url::Url;
use utoipa::ToSchema;

use crate::entities::CustomEmoji;

/// <https://docs.joinmastodon.org/entities/Account/>
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct Account {
    pub id: Url,
    pub username: String,
    pub url: Url,
    pub display_name: String,
    pub avatar: String,
    pub avatar_static: String,
    pub emojis: Vec<CustomEmoji>,
}

impl Account {
    pub fn from_json(user: Service) -> Result<Self, AppError> {
        let avatar = if let Some(icon) = user.icon {
            icon.url.to_string()
        } else {
            format!(
                "https://ui-avatars.com/api/?name={}&background=random&format=svg",
                urlencoding::encode(&user.name)
            )
        };

        Ok(Self {
            id: user.id.clone().into(),
            username: user.preferred_username,
            url: user.id.clone().into(),
            display_name: user.name,
            avatar: avatar.clone(),
            avatar_static: avatar,
            emojis: CustomEmoji::from_json(user.tag),
        })
    }

    pub async fn from_id(user_id: String, data: &Data<AppData>) -> Result<Self, AppError> {
        match User::find_by_id(&user_id).one(&data.conn).await? {
            Some(db_user) => {
                let apub_user: ApubUser = db_user.into();
                let service: Service = apub_user.into_json(data).await?;
                Ok(Self::from_json(service)?)
            },
            None => Err(AppError::not_found("Account", &user_id)),
        }
    }

    pub async fn primary_account(data: &Data<AppData>) -> Result<Self, AppError> {
        let user_id =
            hatsu_utils::url::generate_user_url(data.domain(), &data.env.hatsu_primary_account)?
                .to_string();

        Self::from_id(user_id, data).await
    }
}
