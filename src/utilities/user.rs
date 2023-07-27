use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
// use url::Url;

use crate::{
    error::Error,
    // utilities::absolutize_relative_url,
};

/// User Site Feed
/// TODO: Option<String> => Option<Url>
#[derive(Deserialize, Serialize)]
pub struct Feed {
    pub json: Option<String>,
    // pub atom: Option<String>,
    // pub rss: Option<String>,
}

/// 从网站获取 Feed 链接
pub async fn get_site_feed(domain: String) -> Result<Feed, Error> {
    let response = reqwest::get(format!("https://{}", &domain)).await?;
    let text = response.text().await?;
    let document = Html::parse_document(&text);

    fn feed_auto_discovery(document: &Html, domain: &str, kind: &str) -> Result<Option<String>, Error> {
        let selector = Selector::parse(&format!("link[rel=\"alternate\"][type=\"{}\"]", kind)).unwrap();
        let link = document.select(&selector)
            .next()
            .unwrap()
            .value()
            .attr("href")
            .unwrap()
            .to_string();
        // let absolute_link = absolutize_relative_url(Url::parse(&link)?, domain.to_string())?.to_string();

        // Ok(absolute_link)
        Ok(Some(link))
    }

    let feed = Feed {
        json: feed_auto_discovery(&document, &domain, "application/feed+json")?,
        // atom: feed_auto_discovery(&document, &domain,"application/atom+xml")?,
        // rss: feed_auto_discovery(&document, &domain, "application/rss+xml")?,
    };

    Ok(feed)
}