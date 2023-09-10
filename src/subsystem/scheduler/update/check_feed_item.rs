use activitypub_federation::{kinds::public, config::Data};
use sea_orm::*;
use url::Url;

use crate::{
    AppData,
    AppError,
    entities::{
        prelude::*,
        post::Model as DbPost,
        user::Model as DbUser,
        user_feed_item::Model as DbUserFeedItem,
    },
    protocol::{objects::Note, activities::{CreateOrUpdateNote, CreateOrUpdateType}},
};

pub async fn check_feed_item(data: &Data<AppData>, user: &DbUser, item: DbUserFeedItem) -> Result<(), AppError> {
    match UserFeedItem::find_by_id(&item.id)
        .one(&data.conn)
        .await? {
            Some(prev_item) => {
                // item.date_modified 不为空且不等于 prev_item.date_modified
                item.date_modified.map(|date_modified| {
                    match prev_item.date_modified {
                        Some(prev_date_modified) if prev_date_modified != date_modified => {
                            // TODO: Update Post
                        },
                        None => {
                            // TODO: Update Post
                        },
                        _ => {}
                    }
                });

                Ok(())
            }
            None => {
                // 创建 Note
                let note = Note::new(
                    Url::parse(&format!("https://{}/o/{}", data.domain(), item.id))?.into(),
                    user,
                    format!(
                        "{}\n{}\n{}",
                        // TODO: fallback
                        item.title.unwrap_or_default(),
                        item.summary.unwrap_or_default(),
                        item.id
                    ),
                    &data
                )?;

                // 创建 Post 并保存到数据库
                let _post = DbPost {
                    id: note.id.to_string(),
                    attributed_to: note.attributed_to.to_string(),
                    object: serde_json::to_string(&note)?,
                    published: note.published.clone(),
                    updated: note.updated.clone(),
                    last_refreshed_at: note.published.clone().unwrap(),
                    local: true,
                }
                    .into_active_model()
                    .insert(&data.conn)
                    .await?;

                // 发送 Note
                user.send_activity(CreateOrUpdateNote::new(note, CreateOrUpdateType::Create, data).await?, vec![public()], data).await?;

                Ok(())
            }
        }
}
