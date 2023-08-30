use sea_orm::*;
use url::Url;

use crate::{
    AppData,
    AppError,
    entities::{
        prelude::*,
        user::{self, Model as DbUser},
        impls::JsonUserFeed,
    },
    utilities::Feed,
};

pub async fn fast_update(data: &AppData) -> Result<(), AppError> {
    for user in User::find()
        .filter(user::Column::Local.eq(true))
        .order_by_asc(user::Column::Id)
        .all(&data.conn)
        .await? {
            fast_update_partial(user, data).await?;
        }

    Ok(())
}

async fn fast_update_partial(user: DbUser, data: &AppData) -> Result<(), AppError> {
    let feed: Feed = serde_json::from_str(&user.feed.unwrap())?;

    // Tests for JSON Feed only
    match UserFeed::find_by_id(&feed.json.clone().unwrap())
        .one(&data.conn)
        .await? {
            Some(db_feed) => {
                let curr_feed: JsonUserFeed = reqwest::get(Url::parse(&feed.json.clone().unwrap())?)
                    .await?
                    .json()
                    .await?;

                let prev_feed: JsonUserFeed = db_feed.into_json().await?;

                if prev_feed.items != curr_feed.items {
                    for item in curr_feed.items {
                        // 存在相同 ID
                        if prev_feed.items.iter().map(|item| item.id.clone()).collect::<Vec<String>>().contains(&item.id) {

                        } else { // 不存在，创建帖文

                        }
                    }
                }
            }
            None => {}
    }

    Ok(())
}
