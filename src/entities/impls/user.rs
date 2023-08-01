use std::env;

use activitypub_federation::http_signatures::generate_actor_keypair;
use chrono::Local;
use url::Url;

use crate::{
    entities::user::Model as DbUser,
    error::AppError,
    utilities::get_site_feed,
};

impl DbUser {
  // 创建新用户
  // Create a new user
  // TODO: 从网站获取数据
  // TODO: Getting data from websites
  pub async fn new(preferred_username: &str) -> Result<Self, AppError> {
      let hostname = env::var("HATSU_DOMAIN")?;
      let id = Url::parse(&format!("https://{}/u/{}", hostname, &preferred_username))?;
      let inbox = Url::parse(&format!("https://{}/u/{}/inbox", hostname, &preferred_username))?;
      let outbox = Url::parse(&format!("https://{}/u/{}/outbox", hostname, &preferred_username))?;
      let keypair = generate_actor_keypair()?;

      let feed = get_site_feed(preferred_username.to_string()).await?;

      tracing::info!(
          "User Feed: {}, {}, {}",
          feed.json.unwrap_or_else(|| "null".to_string()),
          feed.atom.unwrap_or_else(|| "null".to_string()),
          feed.rss.unwrap_or_else(|| "null".to_string()),
      );

      Ok(Self {
          id: id.to_string(),
          name: "Hatsu".to_string(),
          preferred_username: preferred_username.to_string(),
          inbox: inbox.to_string(),
          outbox: outbox.to_string(),
          local: true,
          public_key: keypair.public_key,
          private_key: Some(keypair.private_key),
          last_refreshed_at: Local::now().naive_local().format("%Y-%m-%d %H:%M:%S").to_string(),
          // followers: vec![],
      })
  }
}