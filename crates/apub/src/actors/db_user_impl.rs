use activitypub_federation::http_signatures::generate_actor_keypair;
use chrono::{Local, SecondsFormat};
use hatsu_db_schema::user::Model as DbUser;
use hatsu_utils::{
    AppError,
    user::feed::get_site_feed,
};
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
}
