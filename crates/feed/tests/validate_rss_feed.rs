use hatsu_feed::UserFeedTopLevel;
use hatsu_utils::AppError;
use url::Url;

#[tokio::test]
async fn validate_rss_feed() -> Result<(), AppError> {
    UserFeedTopLevel::parse_xml_feed(Url::parse("https://lume.land/blog/feed.xml")?).await?;

    Ok(())
}
