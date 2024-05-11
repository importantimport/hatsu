use hatsu_feed::UserFeed;
use hatsu_utils::AppError;
use url::Url;

#[tokio::test]
async fn validate_rss_feed() -> Result<(), AppError> {
    UserFeed::parse_xml_feed(Url::parse("https://lume.land/blog/feed.xml")?).await?;

    Ok(())
}
