use hatsu_apub::actors::JsonUserFeed;
use hatsu_db_schema::user::Model as DbUser;
use hatsu_utils::AppError;
use url::Url;

pub async fn get_user_feed(user: DbUser) -> Result<JsonUserFeed, AppError> {
    match user {
        DbUser { feed_json: Some(url), .. } => Ok(JsonUserFeed::parse_json_feed(Url::parse(&url)?).await?),
        DbUser { feed_atom: Some(url), .. } => Ok(JsonUserFeed::parse_xml_feed(Url::parse(&url)?).await?),
        DbUser { feed_rss: Some(url), .. } => Ok(JsonUserFeed::parse_xml_feed(Url::parse(&url)?).await?),
        DbUser { feed_json: None, feed_atom: None, feed_rss: None, .. } => Err(AppError::not_found("Feed Url", &user.name))
    }
}
