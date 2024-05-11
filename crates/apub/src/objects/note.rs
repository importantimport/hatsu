// https://github.com/LemmyNet/activitypub-federation-rust/blob/61085a643f05dbb70502b3c519fd666214b7e308/examples/live_federation/objects/post.rs
// https://github.com/LemmyNet/lemmy/blob/main/crates/apub/assets

use activitypub_federation::{
    config::Data,
    fetch::object_id::ObjectId,
    kinds::{link::LinkType, object::NoteType, public},
    protocol::{helpers::deserialize_one_or_many, values::MediaTypeMarkdown},
    traits::{Actor, Object},
};
use hatsu_db_schema::prelude::Post;
use hatsu_feed::UserFeedItem;
use hatsu_utils::{markdown::markdown_to_html, AppData, AppError};
use sea_orm::EntityTrait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use url::Url;
use utoipa::ToSchema;

use crate::{
    actors::ApubUser,
    links::{Hashtag, Tag},
    objects::ApubPost,
};

/// <https://www.w3.org/ns/activitystreams#Note>
#[derive(Deserialize, Serialize, Clone, Debug, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Note {
    #[schema(value_type = Url)]
    pub id: ObjectId<ApubPost>,
    #[schema(value_type = String)]
    #[serde(rename = "type")]
    pub kind: NoteType,
    #[schema(value_type = Option<Url>)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_reply_to: Option<ObjectId<ApubPost>>,
    pub published: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
    #[schema(value_type = Url)]
    pub attributed_to: ObjectId<ApubUser>,
    #[serde(deserialize_with = "deserialize_one_or_many")]
    pub to: Vec<Url>,
    #[serde(deserialize_with = "deserialize_one_or_many")]
    pub cc: Vec<Url>,
    pub content: String,
    /// TODO: customization via item._hatsu.source
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Value>,
    #[serde(default, deserialize_with = "deserialize_one_or_many")]
    pub tag: Vec<Tag>,
    /// <https://www.w3.org/ns/activitystreams#url>
    /// <https://codeberg.org/fediverse/fep/src/branch/main/fep/fffd/fep-fffd.md>
    pub url: Option<Value>,
    // TODO:
    // sensitive (default: false) (extension: _hatsu.sensitive)
    // attachment
    // context (?)
    // conversation (?)
    // license (default: undefined) (extension: _hatsu.license)
}

impl Note {
    // https://example.com/foo/bar => https://example.com/foo/bar
    // /foo/bar => https://example.com/foo/bar
    // foo/bar => https://example.com/foo/bar
    pub fn parse_id(actor: &ApubUser, json: &UserFeedItem) -> Result<Url, AppError> {
        if let Some(url) = &json.url {
            Ok(url.clone())
        } else {
            Ok(hatsu_utils::url::absolutize_relative_url(
                &json.id,
                &actor.preferred_username,
            )?)
        }
    }

    pub fn new(
        actor: &ApubUser,
        json: UserFeedItem,
        published: Option<String>,
        updated: Option<String>,
        data: &Data<AppData>,
    ) -> Result<Self, AppError> {
        // TODO: match json._hatsu.source (string)
        let mut sources: Vec<Option<String>> = vec![json.title.clone(), json.summary.clone()];

        let json_id = Self::parse_id(actor, &json)?;
        sources.push(Some(json_id.to_string()));

        let mut source = sources
            .iter()
            .filter_map(std::clone::Clone::clone)
            .collect::<Vec<String>>()
            .join("\n\n");

        let mut content = markdown_to_html(&source);

        // TODO: json._hatsu.tags (Option<false>)
        if let Some(ref tags) = json.tags {
            source.push_str(&format!(
                "\n\n{}",
                tags.iter()
                    .map(|tag| "#".to_owned() + tag)
                    .collect::<Vec<String>>()
                    .join(" ")
            ));

            content.push_str(&format!(
                "\n\n{}",
                tags
                    .iter()
                    // TODO: test urlencoding::encode()
                    .map(|tag| format!(
                        "<a href=\"https://{}/t/{}\" rel=\"tag\">#<span>{}</span></a>",
                        data.domain(),
                        urlencoding::encode(tag),
                        tag
                    ))
                    .collect::<Vec<String>>()
                    .join(" ")
            ));
        }

        let id = hatsu_utils::url::generate_post_url(data.domain(), json.id)?.into();

        Ok(Self {
            id,
            kind: NoteType::Note,
            in_reply_to: None,
            published: published.unwrap_or_else(hatsu_utils::date::now),
            updated,
            attributed_to: actor.id().into(),
            // to: vec![Url::parse(&format!("{}/followers", actor.id()))?],
            // cc: vec![public()],
            to: vec![public()],
            // Leaving a CC here to retain compatibility, figured I should CC followers instead of public twice
            cc: vec![Url::parse(&format!("{}/followers", actor.id()))?],
            content,
            source: Some(serde_json::to_value(NoteSource::new(source))?),
            tag: json.tags.map_or_else(Vec::new, |tags| {
                tags.into_iter()
                    .map(|tag| {
                        Tag::Hashtag(Hashtag::new(
                            Url::parse(&format!(
                                "https://{}/t/{}",
                                data.domain(),
                                urlencoding::encode(&tag),
                            ))
                            .unwrap(),
                            format!("#{tag}"),
                        ))
                    })
                    .collect()
            }),
            url: Some(serde_json::to_value(NoteUrl::new(json_id))?),
        })
    }

    #[async_recursion::async_recursion]
    pub async fn check_in_reply_to_root(
        self,
        data: &Data<AppData>,
    ) -> Result<Option<String>, AppError> {
        match self.in_reply_to.map(|url| url.to_string()) {
            Some(in_reply_to) if in_reply_to.starts_with(&format!("https://{}", data.domain())) =>
                Ok(Some(in_reply_to)),
            Some(in_reply_to) => match Post::find_by_id(&in_reply_to).one(&data.conn).await? {
                Some(db_post) => {
                    let apub_post: ApubPost = db_post.into();
                    let note = apub_post.into_json(data).await?;

                    Self::check_in_reply_to_root(note, data).await
                },
                None => Ok(None),
            },
            None => Ok(None),
        }
    }

    pub fn create(
        actor: &ApubUser,
        json: UserFeedItem,
        data: &Data<AppData>,
    ) -> Result<Self, AppError> {
        Self::new(actor, json, Some(hatsu_utils::date::now()), None, data)
    }

    pub fn update(
        actor: &ApubUser,
        json: UserFeedItem,
        published: String,
        data: &Data<AppData>,
    ) -> Result<Self, AppError> {
        Self::new(
            actor,
            json,
            Some(published),
            Some(hatsu_utils::date::now()),
            data,
        )
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NoteUrl {
    #[serde(rename = "type")]
    pub kind: LinkType,
    pub rel: String,
    pub href: Url,
}

impl NoteUrl {
    pub fn new(href: Url) -> Self {
        Self {
            kind: LinkType::Link,
            rel: String::from("canonical"),
            href,
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NoteSource {
    pub content: String,
    pub media_type: MediaTypeMarkdown,
}

impl NoteSource {
    pub const fn new(source: String) -> Self {
        Self {
            content: source,
            media_type: MediaTypeMarkdown::Markdown,
        }
    }
}
