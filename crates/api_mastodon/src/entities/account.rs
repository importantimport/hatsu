use activitypub_federation::{
    config::Data,
    traits::Object,
};
use hatsu_apub::actors::{Service, ApubUser};
use hatsu_db_schema::prelude::User;
use hatsu_utils::{AppData, AppError};
use sea_orm::EntityTrait;
use serde::{Deserialize, Serialize};
use url::Url;
use utoipa::ToSchema;

use crate::entities::CustomEmoji;

/// https://docs.joinmastodon.org/entities/Account/
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct Account {
    pub id: Url,
    pub username: String,
    pub url: Url,
    pub display_name: String,
    pub avatar: String,
    pub avatar_static: String,
    /// until I figure it out, it should always be an empty vec
    pub emojis: Vec<CustomEmoji>,
}

impl Account {
    pub fn from_json(
        user: Service,
    ) -> Result<Self, AppError> {
        let avatar = match user.icon {
            Some(icon) => icon.url.to_string(),
            // fallback
            // TODO: update this
            _ => format!("https://ui-avatars.com/api/?name={}&background=random&format=svg", urlencoding::encode(&user.preferred_username)),
        };

        Ok(Self {
            id: user.id.clone().into(),
            username: user.name,
            url: user.id.clone().into(),
            display_name: user.preferred_username,
            avatar: avatar.clone(),
            avatar_static: avatar,
            emojis: vec![],
        })
    }

    // TODO: remove this
    pub async fn primary_account(
        data: &Data<AppData>
    ) -> Result<Self, AppError> {
        match User::find_by_id(format!("https://{}/u/{}", data.domain(), data.env.hatsu_primary_account))
            .one(&data.conn)
            .await? {
                Some(db_user) => {
                    let apub_user: ApubUser = db_user.into();
                    let service: Service = apub_user.into_json(data).await?;
                    Ok(Self::from_json(service)?)
                },
                _ => Err(AppError::not_found("Account", &data.env.hatsu_primary_account))
            }
    }
}
