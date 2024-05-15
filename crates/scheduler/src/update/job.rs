use std::ops::Deref;

use activitypub_federation::config::Data;
use hatsu_apub::actors::ApubUser;
use hatsu_db_schema::{
    prelude::User,
    user::{self, Model as DbUser},
};
use hatsu_feed::{UserFeed, UserFeedTopLevel, WrappedUserFeedItem};
use hatsu_utils::{AppData, AppError};
use sea_orm::{
    ActiveModelTrait,
    ColumnTrait,
    EntityTrait,
    IntoActiveModel,
    QueryFilter,
    QueryOrder,
    Set,
};

use crate::update::check_feed_item;

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

pub async fn fast_update_per_user(data: &Data<AppData>, db_user: DbUser) -> Result<(), AppError> {
    let feed = UserFeedTopLevel::get(db_user.clone()).await?;
    let user: ApubUser = db_user.into();

    for item in feed.items {
        check_feed_item(
            data,
            &user,
            WrappedUserFeedItem::from_json(item, &user, data)?
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

pub async fn full_update_per_user(data: &Data<AppData>, db_user: DbUser) -> Result<(), AppError> {
    let user_feed = UserFeed::get(db_user.preferred_username.to_string()).await?;

    let db_user_feed = Some(user_feed.clone().into_db());

    if !db_user.feed.eq(&db_user_feed) {
        hatsu_db_schema::user::ActiveModel {
            feed: Set(db_user_feed),
            ..db_user.clone().into_active_model()
        }
        .update(&data.conn)
        .await?;
    }

    let user_feed_top_level = user_feed
        .get_top_level(&db_user.preferred_username)
        .await?
        .get_full()
        .await?;

    let user: ApubUser = db_user.into();

    for item in user_feed_top_level.items {
        check_feed_item(
            data,
            &user,
            WrappedUserFeedItem::from_json(item, &user, data)?
                .deref()
                .clone(),
        )
        .await?;
    }

    Ok(())
}
