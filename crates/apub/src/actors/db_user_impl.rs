use activitypub_federation::{
    activity_queue::send_activity,
    config::Data,
    http_signatures::generate_actor_keypair,
    traits::ActivityHandler,
};
use chrono::{Local, SecondsFormat};
use hatsu_db_schema::user::Model as DbUser;
use hatsu_utils::{
    AppData,
    AppError,
    user::feed::get_site_feed,
};
use serde::Serialize;
use url::Url;

use super::{ApubUser, JsonUserFeed};

impl ApubUser {
    pub async fn new(domain: &str, preferred_username: &str) -> Result<Self, AppError> {
        let keypair = generate_actor_keypair()?;

        let feed = get_site_feed(preferred_username.to_string()).await?;

        // TODO: Support Other Feeds
        // Tests for JSON Feed Only
        let json_feed: JsonUserFeed = reqwest::get(Url::parse(&feed.json.clone().unwrap())?)
            .await?
            .json::<JsonUserFeed>()
            .await?;

        let user = DbUser {
            id: format!("https://{}/u/{}", domain, preferred_username),
            name: json_feed.title,
            preferred_username: preferred_username.to_string(),
            summary: json_feed.description,
            icon: json_feed.icon.map(|url| url.to_string()),
            image: json_feed.hatsu.and_then(|hatsu| hatsu.banner_image.map(|url| url.to_string())),
            inbox: format!("https://{}/u/{}/inbox", domain, preferred_username),
            outbox: format!("https://{}/u/{}/outbox", domain, preferred_username),
            followers: format!("https://{}/u/{}/followers", domain, preferred_username),
            following: format!("https://{}/u/{}/following", domain, preferred_username),
            local: true,
            public_key: keypair.public_key,
            private_key: Some(keypair.private_key),
            feed_json: feed.json,
            feed_atom: feed.atom,
            feed_rss: feed.rss,
            last_refreshed_at: Local::now().to_rfc3339_opts(SecondsFormat::Secs, true),
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
    ) -> Result<(), <Activity as ActivityHandler>::Error>
    where
        Activity: ActivityHandler + Serialize,
        <Activity as ActivityHandler>::Error: From<anyhow::Error> + From<serde_json::Error> + From<hatsu_db_migration::DbErr>
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
        send_activity(activity, self, inboxes, data).await?;
        Ok(())
    }
}
