use activitypub_federation::{
    activity_sending::SendActivityTask,
    config::Data,
    http_signatures::generate_actor_keypair,
    traits::ActivityHandler,
};
use chrono::Utc;
use hatsu_db_schema::user::Model as DbUser;
use hatsu_utils::{user::feed::Feed, AppData, AppError};
use serde::Serialize;
use std::fmt::Debug;
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
                .join(&format!("{}/inbox", preferred_username))?
                .to_string(),
            outbox: user_url
                .join(&format!("{}/outbox", preferred_username))?
                .to_string(),
            followers: user_url
                .join(&format!("{}/followers", preferred_username))?
                .to_string(),
            following: user_url
                .join(&format!("{}/following", preferred_username))?
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
    /// activitypub_federation::activity_queue::send_activity 的简单封装
    ///
    /// 遇到类型问题加不上去，不要忘了用 WithContext::new_default(activity) 套一层
    ///
    /// https://github.com/LemmyNet/activitypub-federation-rust/blob/35bf29ae73e33a537a9fdb2d2bb8bb1ba4842991/examples/federation/objects/person.rs#L111-L132
    pub async fn send_activity<Activity>(
        &self,
        activity: Activity,
        inboxes: Vec<Url>,
        data: &Data<AppData>,
    ) -> Result<(), AppError>
    where
        Activity: ActivityHandler + Serialize + Debug,
    {
        // 从 Activity URL 提取 UUID
        // let activity_id: String = activity
        //     .id()
        //     .path()
        //     .split('/')
        //     .last()
        //     .unwrap()
        //     .to_string();

        // 验证这个 UUID
        // let uuid = Uuid::try_parse(&activity_id)?;

        // 保存到数据库
        // activity::Entity::insert(DbActivity {
        //     id: activity_id,
        //     activity: to_string(&activity)?,
        //     actor: activity.actor().to_string(),
        //     kind: activity.kind,

        // }.into_active_model())
        //     .exec(&data.conn)
        //     .await?;

        // 发送
        let sends = SendActivityTask::prepare(&activity, self, inboxes, data).await?;

        for send in sends {
            send.sign_and_send(data).await?;
        }

        Ok(())
    }
}
