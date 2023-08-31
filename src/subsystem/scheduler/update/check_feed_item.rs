use sea_orm::*;
use url::Url;

use crate::{
    AppData,
    AppError,
    entities::{
        prelude::*,
        impls::JsonUserFeedItem,
        user::Model as DbUser,
    },
};

pub async fn check_feed_item(data: &AppData, _user: DbUser, item: JsonUserFeedItem) -> Result<(), AppError> {
    match UserFeedItem::find_by_id(item.url.unwrap_or_else(|| Url::parse(&item.id).unwrap()).to_string())
        .one(&data.conn)
        .await? {
            Some(prev_item) => {
                // item.date_modified 不为空且不等于 prev_item.date_modified
                item.date_modified.and_then(|date_modified| {
                    match prev_item.date_modified {
                        Some(prev_date_modified) if prev_date_modified != date_modified => {
                            // TODO: Update Post
                        },
                        None => {
                            // TODO: Update Post
                        },
                        _ => {}
                    }
                    
                    Some(1)
                });

                Ok(())
            }
            None => {
                // TODO: Create Post
                Ok(())
            }
        }
}
