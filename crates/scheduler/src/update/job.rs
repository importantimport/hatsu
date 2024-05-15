use std::ops::Deref;

use activitypub_federation::config::Data;
use hatsu_apub::actors::ApubUser;
use hatsu_db_schema::{
    prelude::User,
    user::{self, Model as DbUser},
};
use hatsu_feed::{UserFeed, UserFeedHatsu, UserFeedTopLevel, WrappedUserFeedItem};
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
use url::Url;

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
    let mut db_user = db_user;

    let user_feed = UserFeed::get(db_user.preferred_username.to_string()).await?;

    let db_user_feed = Some(user_feed.clone().into_db());

    if !db_user.feed.eq(&db_user_feed) {
        db_user = hatsu_db_schema::user::ActiveModel {
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

    if !Into::<ApubUser>::into(db_user.clone())
        .to_user_feed_top_level()
        .eq(&UserFeedTopLevel {
            // TODO: use language
            language: Option::default(),
            // Default::default()
            feed_url: Url::parse("https://hatsu.local").unwrap(),
            next_url: Option::default(),
            items: Vec::default(),
            ..user_feed_top_level.clone()
        })
    {
        db_user = hatsu_db_schema::user::ActiveModel {
            hatsu: Set(user_feed_top_level.hatsu.map(UserFeedHatsu::into_db)),
            name: Set(user_feed_top_level.title),
            summary: Set(user_feed_top_level.description),
            icon: Set(user_feed_top_level.icon.map(|url| url.to_string())),
            ..db_user.clone().into_active_model()
        }
        .update(&data.conn)
        .await?;
    }

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
