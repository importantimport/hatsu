use std::fmt::Debug;

use activitypub_federation::{
    activity_sending::SendActivityTask,
    config::Data,
    fetch::object_id::ObjectId,
    http_signatures::generate_actor_keypair,
    traits::{ActivityHandler, Actor},
};
use chrono::Utc;
use hatsu_db_schema::{prelude::ReceivedFollow, user::Model as DbUser};
use hatsu_utils::{user::feed::Feed, AppData, AppError};
use sea_orm::ModelTrait;
use serde::Serialize;
use url::Url;

use super::{ApubUser, JsonUserFeed};

impl ApubUser {
    pub async fn new(domain: &str, preferred_username: &str) -> Result<Self, AppError> {
        let keypair = generate_actor_keypair()?;

        let user_feed = Feed::get_site_feed(preferred_username.to_string()).await?;

        let feed = JsonUserFeed::get_feed(user_feed.clone(), preferred_username).await?;

        let user_url = hatsu_utils::url::generate_user_url(domain, preferred_username)?;

        let user = DbUser {
            id: user_url.to_string(),
            name: feed.title,
            preferred_username: preferred_username.to_string(),
            summary: feed.description,
            icon: feed.icon.map(|url| url.to_string()),
            image: feed
                .hatsu
                .and_then(|hatsu| hatsu.banner_image.map(|url| url.to_string())),
            // TODO: test this
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
            feed_json: user_feed.json.map(|url| url.to_string()),
            feed_atom: user_feed.atom.map(|url| url.to_string()),
            feed_rss: user_feed.rss.map(|url| url.to_string()),
            last_refreshed_at: Utc::now().to_rfc3339(),
        };

        Ok(user.into())
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
        let inboxes = match inboxes {
            Some(inboxes) => inboxes,
            None => {
                // 获取 followers inbox
                let handles = self
                    .find_related(ReceivedFollow)
                    .all(&data.conn)
                    .await?
                    .into_iter()
                    .map(|received_follow| async move {
                        let follower: ObjectId<ApubUser> =
                            Url::parse(&received_follow.actor).unwrap().into();
                        let follower: ApubUser = follower.dereference_local(data).await.unwrap();
                        follower.shared_inbox_or_inbox()
                    })
                    .collect::<Vec<_>>();

                futures::future::join_all(handles).await
            }
        };

        // 发送
        let sends = SendActivityTask::prepare(&activity, self, inboxes, data).await?;

        for send in sends {
            send.sign_and_send(data).await?;
        }

        Ok(())
    }
}
