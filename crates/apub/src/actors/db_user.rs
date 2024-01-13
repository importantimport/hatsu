use std::ops::Deref;

use activitypub_federation::{
    config::Data,
    protocol::verification::verify_domains_match,
    traits::{Actor, Object},
};
use chrono::{DateTime, Utc};
use hatsu_db_schema::{
    prelude::User,
    user::{self, Model as DbUser},
};
use hatsu_utils::{AppData, AppError};
use sea_orm::{EntityTrait, IntoActiveModel, sea_query};
use url::Url;

use crate::actors::{Service, ServiceImage};

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
    type Kind = Service;

    fn last_refreshed_at(&self) -> Option<DateTime<Utc>> {
        Some(
            DateTime::parse_from_rfc3339(&self.last_refreshed_at)
                .unwrap()
                .into(),
        )
    }

    async fn read_from_id(
        object_id: Url,
        data: &Data<Self::DataType>,
    ) -> Result<Option<Self>, Self::Error> {
        Ok(User::find_by_id(&object_id.to_string())
            .one(&data.conn)
            .await?
            .map(Into::into))
    }

    async fn delete(self, data: &Data<Self::DataType>) -> Result<(), Self::Error> {
        let _delete_user = User::delete_by_id(&self.id.to_string())
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
            image: json.image.map(|image| image.url.to_string()),
            inbox: json.inbox.to_string(),
            outbox: json.outbox.to_string(),
            followers: json.followers.to_string(),
            following: json.following.to_string(),
            local: false,
            public_key: json.public_key.public_key_pem,
            private_key: None,
            feed_json: None,
            feed_atom: None,
            feed_rss: None,
            last_refreshed_at: Utc::now().to_rfc3339(),
        };

        // 写入数据库
        // TODO: on_conflict 时执行更新
        User::insert(user.clone().into_active_model())
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

    async fn into_json(self, _data: &Data<Self::DataType>) -> Result<Self::Kind, Self::Error> {
        Ok(Service {
            kind: Default::default(),
            name: self.name.clone(),
            preferred_username: self.preferred_username.clone(),
            id: Url::parse(&self.id)?.into(),
            summary: self.summary.clone(),
            icon: self.icon.clone().map(|icon| ServiceImage {
                kind: Default::default(),
                url: Url::parse(&icon).unwrap(),
            }),
            image: self.image.clone().map(|image| ServiceImage {
                kind: Default::default(),
                url: Url::parse(&image).unwrap(),
            }),
            // TODO: User Attachment
            attachment: vec![],
            inbox: Url::parse(&self.inbox)?,
            outbox: Url::parse(&self.outbox)?,
            followers: Url::parse(&self.followers)?,
            following: Url::parse(&self.following)?,
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
