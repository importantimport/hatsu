use std::ops::Deref;

use activitypub_federation::config::Data;
use hatsu_apub::actors::{ApubUser, ApubUserFeedItem, JsonUserFeed};
use hatsu_db_schema::{
    prelude::User,
    user::{self, Model as DbUser},
};
use hatsu_utils::{AppData, AppError};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QueryOrder};

use crate::update::{check_feed_item, get_user_feed};

pub async fn fast_update(data: &Data<AppData>) -> Result<(), AppError> {
    for user in User::find()
        .filter(user::Column::Local.eq(true))
        .order_by_asc(user::Column::Id)
        .all(&data.conn)
        .await?
    {
        fast_update_per_user(data, user).await?;
    }

    Ok(())
}

pub async fn fast_update_per_user(data: &Data<AppData>, user: DbUser) -> Result<(), AppError> {
    let feed: JsonUserFeed = get_user_feed(user.clone()).await?;
    let user: ApubUser = user.into();

    for item in feed.items {
        check_feed_item(
            data,
            &user,
            ApubUserFeedItem::from_json(item, &user, data)?
                .deref()
                .clone(),
        )
        .await?;
    }

    Ok(())
}

pub async fn full_update(data: &Data<AppData>) -> Result<(), AppError> {
    for user in User::find()
        .filter(user::Column::Local.eq(true))
        .order_by_asc(user::Column::Id)
        .all(&data.conn)
        .await?
    {
        full_update_per_user(data, user).await?;
    }

    Ok(())
}

pub async fn full_update_per_user(data: &Data<AppData>, user: DbUser) -> Result<(), AppError> {
    let feed: JsonUserFeed = get_user_feed(user.clone()).await?.get_full_feed().await?;
    let user: ApubUser = user.into();

    for item in feed.items {
        check_feed_item(
            data,
            &user,
            ApubUserFeedItem::from_json(item, &user, data)?
                .deref()
                .clone(),
        )
        .await?;
    }

    Ok(())
}
