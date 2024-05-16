use std::fmt::Debug;

use activitypub_federation::{
    activity_queue::queue_activity,
    config::Data,
    fetch::object_id::ObjectId,
    http_signatures::generate_actor_keypair,
    traits::{ActivityHandler, Actor},
};
use hatsu_db_schema::{prelude::ReceivedFollow, user::Model as DbUser};
use hatsu_feed::{UserFeed, UserFeedHatsu, UserFeedTopLevel};
use hatsu_utils::{AppData, AppError};
use sea_orm::ModelTrait;
use serde::Serialize;
use url::Url;

use super::ApubUser;

impl ApubUser {
    pub async fn new(domain: &str, preferred_username: &str) -> Result<Self, AppError> {
        let keypair = generate_actor_keypair()?;

        let user_feed = UserFeed::get(preferred_username.to_string()).await?;

        let user_feed_top_level =
            UserFeed::get_top_level(user_feed.clone(), preferred_username).await?;

        let user_url = hatsu_utils::url::generate_user_url(domain, preferred_username)?;

        let user = DbUser {
            id: user_url.to_string(),
            name: user_feed_top_level.title,
            preferred_username: preferred_username.to_string(),
            summary: user_feed_top_level.description,
            icon: user_feed_top_level.icon.map(|url| url.to_string()),
            inbox: user_url
                .join(&format!("{preferred_username}/inbox"))?
                .to_string(),
            outbox: user_url
                .join(&format!("{preferred_username}/outbox"))?
                .to_string(),
            followers: user_url
                .join(&format!("{preferred_username}/followers"))?
                .to_string(),
            following: user_url
                .join(&format!("{preferred_username}/following"))?
                .to_string(),
            local: true,
            public_key: keypair.public_key,
            private_key: Some(keypair.private_key),
            hatsu: user_feed_top_level.hatsu.map(UserFeedHatsu::into_db),
            feed: Some(user_feed.into_db()),
            last_refreshed_at: hatsu_utils::date::now(),
        };

        Ok(user.into())
    }

    /// For `hatsu_scheduler::update::full_update`
    #[must_use]
    pub fn to_user_feed_top_level(self) -> UserFeedTopLevel {
        UserFeedTopLevel {
            hatsu: self.hatsu.clone().map(UserFeedHatsu::from_db),
            title: self.name.clone(),
            description: self.summary.clone(),
            icon: self.icon.clone().and_then(|url| Url::parse(&url).ok()),
            // TOOD: use language
            language: Option::default(),
            // Default::default()
            feed_url: Url::parse("https://hatsu.local").unwrap(),
            next_url: Option::default(),
            items: Vec::default(),
        }
    }

    /// 发送动态 / Send Activity
    ///
    /// `activitypub_federation::activity_queue::send_activity` 的简单封装
    ///
    /// 遇到类型问题加不上去，不要忘了用 `WithContext::new_default(activity`) 套一层
    ///
    /// <https://github.com/LemmyNet/activitypub-federation-rust/blob/35bf29ae73e33a537a9fdb2d2bb8bb1ba4842991/examples/federation/objects/person.rs#L111-L132>
    pub async fn send_activity<Activity>(
        &self,
        activity: Activity,
        inboxes: Option<Vec<Url>>,
        data: &Data<AppData>,
    ) -> Result<(), AppError>
    where
        Activity: ActivityHandler + Serialize + Debug,
    {
        let inboxes = if let Some(inboxes) = inboxes {
            inboxes
        } else {
            // 获取 followers inbox
            let handles = self
                .find_related(ReceivedFollow)
                .all(&data.conn)
                .await?
                .into_iter()
                .map(|received_follow| async move {
                    let follower: ObjectId<Self> =
                        Url::parse(&received_follow.actor).unwrap().into();
                    let follower: Self = follower.dereference_local(data).await.unwrap();
                    follower.shared_inbox_or_inbox()
                })
                .collect::<Vec<_>>();

            futures::future::join_all(handles).await
        };

        // 发送
        queue_activity(&activity, self, inboxes, data).await?;

        Ok(())
    }
}
