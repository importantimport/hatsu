use activitypub_federation::config::Data;
use sea_orm::*;
use url::Url;

use crate::{
    AppData,
    AppError,
    entities::{
        prelude::*,
        impls::JsonUserFeed,
        user::{self, Model as DbUser},
        user_feed_item::Model as DbUserFeedItem,
    },
    subsystem::scheduler::update::get_user_feed,
};

use super::check_feed_item;

pub async fn fast_update(data: &Data<AppData>) -> Result<(), AppError> {
    for user in User::find()
        .filter(user::Column::Local.eq(true))
        .order_by_asc(user::Column::Id)
        .all(&data.conn)
        .await? {
            fast_update_per_user(data, user).await?;
        }

    Ok(())
}

pub async fn fast_update_per_user(data: &Data<AppData>, user: DbUser) -> Result<(), AppError> {
    let feed: JsonUserFeed = get_user_feed(user.clone()).await?;

    for item in feed.items {
        // check_feed_item(data, &user, DbUserFeedItem::from_json(item, Url::parse(&user.id)?.into(), None).await?).await?;
        check_feed_item(data, &user, DbUserFeedItem::from_json(item, Url::parse(&user.id)?.into(), data)?).await?;
    }

    Ok(())
}