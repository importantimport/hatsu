use chrono::SecondsFormat;
use feed_rs::parser;
use hatsu_apub::actors::{JsonUserFeed, JsonUserFeedItem};
use hatsu_db_schema::user::Model as DbUser;
use hatsu_utils::AppError;
use url::Url;

pub async fn get_user_feed(user: DbUser) -> Result<JsonUserFeed, AppError> {
    match user {
        DbUser { feed_json: Some(url), .. } => Ok(parse_json_feed(url).await?),
        DbUser { feed_atom: Some(url), .. } => Ok(parse_xml_feed(url).await?),
        DbUser { feed_rss: Some(url), .. } => Ok(parse_xml_feed(url).await?),
        DbUser { feed_json: None, feed_atom: None, feed_rss: None, .. } => Err(AppError::not_found("Feed Url", &user.name))
    }
}

async fn parse_json_feed(url: String) -> Result<JsonUserFeed, AppError> {
    Ok(reqwest::get(Url::parse(&url)?)
        .await?
        .json::<JsonUserFeed>()
        .await?)
}

async fn parse_xml_feed(url: String) -> Result<JsonUserFeed, AppError> {
    let feed = parser::parse(
        reqwest::get(Url::parse(&url)?)
            .await?
            .text()
            .await?
            .as_bytes()
    )?;
        
    let items = feed.entries
        .iter()
        .map(|entry| JsonUserFeedItem {
            id: entry.id.clone(),
            url: None, // TODO
            title: entry.title.clone().map(|text| text.content),
            summary: entry.summary.clone().map(|text| text.content),
            language: None,
            tags: entry.categories.iter().map(|category| Some(category.label.clone().unwrap_or(category.term.clone()))).collect(),
            date_published: entry.published.map(|date| date.to_rfc3339_opts(SecondsFormat::Secs, true)),
            date_modified: entry.updated.map(|date| date.to_rfc3339_opts(SecondsFormat::Secs, true)),
        })
        .collect();

    Ok(JsonUserFeed {
        hatsu: None,
        feed_url: Url::parse(&feed.id).unwrap(),
        next_url: None,
        title: feed.title.unwrap().content,
        description: feed.description.map(|text| text.content),
        icon: feed.icon.map(|image| Url::parse(&image.uri).unwrap()),
        language: feed.language,
        items,
    })
}
