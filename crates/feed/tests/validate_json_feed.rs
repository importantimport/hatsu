use hatsu_feed::UserFeedTopLevel;
use hatsu_utils::AppError;
use url::Url;

#[tokio::test]
async fn validate_json_feed() -> Result<(), AppError> {
    UserFeedTopLevel::parse_json_feed(Url::parse("https://lume.land/blog/feed.json")?).await?;

    Ok(())
}
