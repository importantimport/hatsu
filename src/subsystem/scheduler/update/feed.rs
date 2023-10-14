use chrono::SecondsFormat;
use feed_rs::parser;
use url::Url;

use crate::{
    AppError,
    entities::{
        impls::{JsonUserFeed, JsonUserFeedItem},
        user::Model as DbUser,
    },
};

pub async fn get_user_feed(user: DbUser) -> Result<JsonUserFeed, AppError> {
    match user {
        DbUser { feed_json: Some(url), .. } => Ok(parse_json_feed(url).await?),
        DbUser { feed_atom: Some(url), .. } => Ok(parse_xml_feed(url).await?),
        DbUser { feed_rss: Some(url), .. } => Ok(parse_xml_feed(url).await?),
        DbUser { feed_json: None, feed_atom: None, feed_rss: None, .. } => Err(AppError::NotFound { kind: "Feed Url".to_string(), name: user.name })
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
            title: entry.title.clone().and_then(|text| Some(text.content)),
            summary: entry.summary.clone().and_then(|text| Some(text.content)),
            language: None,
            tags: entry.categories.iter().map(|category| Some(category.label.clone().unwrap_or(category.term.clone()))).collect(),
            date_published: entry.published.and_then(|date| Some(date.to_rfc3339_opts(SecondsFormat::Secs, true))),
            date_modified: entry.updated.and_then(|date| Some(date.to_rfc3339_opts(SecondsFormat::Secs, true))),
        })
        .collect();

    Ok(JsonUserFeed {
        hatsu: None,
        feed_url: Url::parse(&feed.id).unwrap(),
        next_url: None,
        title: feed.title.unwrap().content,
        description: feed.description.and_then(|text| Some(text.content)),
        icon: feed.icon.and_then(|image| Some(Url::parse(&image.uri).unwrap())),
        language: feed.language,
        items,
    })
}
