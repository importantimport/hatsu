//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.1

use sea_orm::{FromJsonQueryResult, entity::prelude::*};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub name: String,
    pub preferred_username: String,
    pub summary: Option<String>,
    pub icon: Option<String>,
    pub inbox: String,
    pub outbox: String,
    pub followers: String,
    pub following: String,
    pub local: bool,
    pub public_key: String,
    pub private_key: Option<String>,
    pub last_refreshed_at: String,
    pub hatsu: Option<UserHatsu>,
    pub feed: Option<UserFeed>,
    pub language: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, FromJsonQueryResult)]
pub struct UserFeed {
    pub json: Option<String>,
    pub atom: Option<String>,
    pub rss: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize, FromJsonQueryResult)]
pub struct UserHatsu {
    pub about: Option<String>,
    pub aliases: Option<String>,
    pub banner_image: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::activity::Entity")]
    Activity,
    #[sea_orm(has_many = "super::post::Entity")]
    Post,
    #[sea_orm(has_many = "super::received_announce::Entity")]
    ReceivedAnnounce,
    #[sea_orm(has_many = "super::received_follow::Entity")]
    ReceivedFollow,
    #[sea_orm(has_many = "super::received_like::Entity")]
    ReceivedLike,
    #[sea_orm(has_many = "super::user_feed_item::Entity")]
    UserFeedItem,
}

impl ActiveModelBehavior for ActiveModel {}

impl Related<super::activity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Activity.def()
    }
}

impl Related<super::post::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Post.def()
    }
}

impl Related<super::received_announce::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ReceivedAnnounce.def()
    }
}

impl Related<super::received_follow::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ReceivedFollow.def()
    }
}

impl Related<super::received_like::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ReceivedLike.def()
    }
}

impl Related<super::user_feed_item::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserFeedItem.def()
    }
}
