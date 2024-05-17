use std::ops::Deref;

use activitypub_federation::{
    config::Data,
    kinds::actor::ServiceType,
    protocol::verification::verify_domains_match,
    traits::{Actor, Object},
};
use chrono::{DateTime, Utc};
use hatsu_db_schema::{
    prelude::User as PreludeUser,
    user::{self, Model as DbUser},
};
use hatsu_utils::{AppData, AppError};
use sea_orm::{sea_query, EntityTrait, IntoActiveModel};
use url::Url;

use crate::actors::{User, UserAttachment, UserImage, UserType};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ApubUser(pub(crate) DbUser);

impl AsRef<DbUser> for ApubUser {
    fn as_ref(&self) -> &DbUser {
        &self.0
    }
}

impl Deref for ApubUser {
    type Target = DbUser;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<DbUser> for ApubUser {
    fn from(u: DbUser) -> Self {
        Self(u)
    }
}

#[async_trait::async_trait]
impl Object for ApubUser {
    type DataType = AppData;
    type Error = AppError;
    type Kind = User;

    fn last_refreshed_at(&self) -> Option<DateTime<Utc>> {
        hatsu_utils::date::parse(&self.last_refreshed_at).ok()
    }

    async fn read_from_id(
        user_id: Url,
        data: &Data<Self::DataType>,
    ) -> Result<Option<Self>, Self::Error> {
        Ok(PreludeUser::find_by_id(&user_id.to_string())
            .one(&data.conn)
            .await?
            .map(Into::into))
    }

    async fn delete(self, data: &Data<Self::DataType>) -> Result<(), Self::Error> {
        let _delete_user = PreludeUser::delete_by_id(&self.id.to_string())
            .exec(&data.conn)
            .await?;
        Ok(())
    }

    async fn verify(
        json: &Self::Kind,
        expected_domain: &Url,
        _data: &Data<Self::DataType>,
    ) -> Result<(), Self::Error> {
        verify_domains_match(json.id.inner(), expected_domain)?;
        Ok(())
    }

    async fn from_json(json: Self::Kind, data: &Data<Self::DataType>) -> Result<Self, Self::Error> {
        let user = DbUser {
            id: json.id.to_string(),
            name: json.name,
            preferred_username: json.preferred_username,
            summary: json.summary,
            icon: json.icon.map(|icon| icon.url.to_string()),
            inbox: json.inbox.to_string(),
            outbox: json.outbox.to_string(),
            followers: json.followers.to_string(),
            following: json.following.to_string(),
            local: false,
            public_key: json.public_key.public_key_pem,
            private_key: None,
            hatsu: None,
            feed: None,
            last_refreshed_at: hatsu_utils::date::now(),
        };

        // 写入数据库
        // TODO: on_conflict 时执行更新
        PreludeUser::insert(user.clone().into_active_model())
            .on_conflict(
                sea_query::OnConflict::column(user::Column::Id)
                    .do_nothing()
                    .to_owned(),
            )
            .do_nothing()
            .exec(&data.conn)
            .await?;

        Ok(user.into())
    }

    async fn into_json(self, data: &Data<Self::DataType>) -> Result<Self::Kind, Self::Error> {
        let aliases = self
            .hatsu
            .clone()
            .and_then(|hatsu| hatsu.aliases)
            .unwrap_or_else(|| self.preferred_username.clone());

        let domain = Url::parse(&format!("https://{}", &self.preferred_username))?;

        Ok(User {
            kind: UserType::ServiceType(ServiceType::Service),
            name: self.name.clone(),
            preferred_username: self.preferred_username.clone(),
            id: Url::parse(&self.id)?.into(),
            summary: self.summary.clone(),
            icon: self
                .icon
                .clone()
                .map(|icon| UserImage::new(Url::parse(&icon).unwrap())),
            image: self.hatsu.clone().and_then(|hatsu| {
                hatsu
                    .banner_image
                    .map(|image| UserImage::new(Url::parse(&image).unwrap()))
            }),
            attachment: self
                .feed
                .clone()
                .map_or(Vec::new(), |feed| UserAttachment::generate(&domain, feed)),
            inbox: Url::parse(&self.inbox)?,
            outbox: Url::parse(&self.outbox)?,
            followers: Url::parse(&self.followers)?,
            following: Url::parse(&self.following)?,
            tag: vec![],
            // FEP-4adb
            aliases: Some(vec![
                format!("acct:{}@{}", &aliases, &self.preferred_username),
                format!("acct:{}@{}", &self.preferred_username, data.domain()),
            ]),
            // FEP-2c59
            webfinger: Some(format!("acct:{}@{}", &aliases, &self.preferred_username)),
            public_key: self.public_key(),
        })
    }
}

impl Actor for ApubUser {
    fn id(&self) -> Url {
        Url::parse(&self.id).unwrap()
    }

    fn public_key_pem(&self) -> &str {
        &self.public_key
    }

    fn private_key_pem(&self) -> Option<String> {
        self.private_key.clone()
    }

    fn inbox(&self) -> Url {
        Url::parse(&self.inbox).unwrap()
    }
}
