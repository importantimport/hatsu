use activitypub_federation::config::Data;
use hatsu_apub::{activities::CreateOrUpdateNote, actors::ApubUser, objects::Note};
use hatsu_db_schema::{
    post::{self, Model as DbPost},
    prelude::*,
    user_feed_item::Model as DbUserFeedItem,
};
use hatsu_feed::{UserFeedItem as JsonUserFeedItem, WrappedUserFeedItem};
use hatsu_utils::{AppData, AppError};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, EntityTrait, IntoActiveModel};

pub async fn check_feed_item(
    data: &Data<AppData>,
    user: &ApubUser,
    item: DbUserFeedItem,
) -> Result<(), AppError> {
    match UserFeedItem::find_by_id(&item.id).one(&data.conn).await? {
        Some(prev_item) => {
            // item.date_modified 不为空且不等于 prev_item.date_modified
            if let Some(date_modified) = item.date_modified.clone() {
                match prev_item.date_modified {
                    Some(prev_date_modified) if prev_date_modified != date_modified =>
                        update_feed_item(item, user, data).await?,
                    None => update_feed_item(item, user, data).await?,
                    _ => (),
                };
            }

            Ok(())
        },
        None => Ok(create_feed_item(item, user, data).await?),
    }
}

async fn create_feed_item(
    item: DbUserFeedItem,
    user: &ApubUser,
    data: &Data<AppData>,
) -> Result<(), AppError> {
    // 将 Item 保存到数据库
    let item = item.into_active_model().insert(&data.conn).await?;
    let item: WrappedUserFeedItem = item.into();

    // 创建 Note
    let note = Note::create(user, item.into_json()?, data)?;

    // 创建 Post 并保存到数据库
    let _post = DbPost {
        id: note.id.to_string(),
        attributed_to: note.attributed_to.to_string(),
        in_reply_to: None,
        in_reply_to_root: None,
        object: serde_json::to_string(&note)?,
        published: note.published.clone(),
        updated: note.updated.clone(),
        last_refreshed_at: note.published.clone(),
        local: true,
    }
    .into_active_model()
    .insert(&data.conn)
    .await?;

    // 发送 Note
    user.send_activity(CreateOrUpdateNote::create(note, data).await?, None, data)
        .await?;

    Ok(())
}

async fn update_feed_item(
    item: DbUserFeedItem,
    user: &ApubUser,
    data: &Data<AppData>,
) -> Result<(), AppError> {
    // 更新 Item
    let item = item
        .into_active_model()
        .reset_all()
        .update(&data.conn)
        .await?;
    let item: WrappedUserFeedItem = item.into();
    let item: JsonUserFeedItem = item.into_json()?;

    // 更新 Post
    if let Some(post) = Post::find_by_id(Note::parse_id(user, &item)?)
        .one(&data.conn)
        .await?
    {
        let note = Note::update(user, item, post.published.clone(), data)?;

        post::ActiveModel {
            object: Set(serde_json::to_string(&note)?),
            updated: Set(note.updated.clone()),
            last_refreshed_at: Set(hatsu_utils::date::now()),
            ..post.into_active_model()
        }
        .update(&data.conn)
        .await?;

        user.send_activity(CreateOrUpdateNote::update(note, data).await?, None, data)
            .await?;
    }

    Ok(())
}
