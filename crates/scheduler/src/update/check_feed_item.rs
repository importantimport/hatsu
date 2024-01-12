use activitypub_federation::{config::Data, kinds::public};
use hatsu_apub::{
    activities::{CreateOrUpdateNote, CreateOrUpdateType},
    actors::{ApubUser, ApubUserFeedItem},
    objects::Note,
};
use hatsu_db_schema::{
    post::Model as DbPost,
    prelude::UserFeedItem,
    user_feed_item::Model as DbUserFeedItem,
};
use hatsu_utils::{AppData, AppError};
use sea_orm::*;

pub async fn check_feed_item(
    data: &Data<AppData>,
    user: &ApubUser,
    item: DbUserFeedItem,
) -> Result<(), AppError> {
    match UserFeedItem::find_by_id(&item.id).one(&data.conn).await? {
        Some(prev_item) => {
            // item.date_modified 不为空且不等于 prev_item.date_modified
            if let Some(date_modified) = item.date_modified {
                match prev_item.date_modified {
                    Some(prev_date_modified) if prev_date_modified != date_modified => {
                        // TODO: Update Post
                    }
                    None => {
                        // TODO: Update Post
                    }
                    _ => {}
                }
            }

            Ok(())
        }
        None => Ok(create_feed_item(data, user, item).await?),
    }
}

async fn create_feed_item(
    data: &Data<AppData>,
    user: &ApubUser,
    item: DbUserFeedItem,
) -> Result<(), AppError> {
    // 将 Item 保存到数据库
    let item = item.into_active_model().insert(&data.conn).await?;
    let item: ApubUserFeedItem = item.into();

    // 创建 Note
    let note = Note::new(user, item.into_json()?, data)?;

    // 创建 Post 并保存到数据库
    let _post = DbPost {
        id: note.id.to_string(),
        attributed_to: note.attributed_to.to_string(),
        in_reply_to: None,
        in_reply_to_root: None,
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
    // TODO: check public()?
    user.send_activity(
        CreateOrUpdateNote::new(note, CreateOrUpdateType::Create, data).await?,
        vec![public()],
        data,
    )
    .await?;

    Ok(())
}
