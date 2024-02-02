// https://github.com/LemmyNet/activitypub-federation-rust/blob/61085a643f05dbb70502b3c519fd666214b7e308/examples/live_federation/objects/post.rs
// https://github.com/LemmyNet/lemmy/blob/main/crates/apub/assets

use activitypub_federation::{
    config::Data,
    fetch::object_id::ObjectId,
    kinds::{object::NoteType, public},
    protocol::helpers::deserialize_one_or_many,
    traits::{Actor, Object},
};
use hatsu_db_schema::prelude::Post;
use hatsu_utils::{markdown::markdown_to_html, AppData, AppError};
use sea_orm::EntityTrait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use url::Url;

use crate::{
    actors::{ApubUser, JsonUserFeedItem},
    links::Hashtag,
    objects::ApubPost,
};

/// <https://www.w3.org/ns/activitystreams#Note>
#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Note {
    pub id: ObjectId<ApubPost>,
    #[serde(rename = "type")]
    pub kind: NoteType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_reply_to: Option<ObjectId<ApubPost>>,
    pub published: String,
    // pub url: String,
    pub attributed_to: ObjectId<ApubUser>,
    #[serde(deserialize_with = "deserialize_one_or_many")]
    pub to: Vec<Url>,
    #[serde(deserialize_with = "deserialize_one_or_many")]
    pub cc: Vec<Url>,
    pub content: String,
    /// TODO: customization via item._hatsu.source
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
    // TODO:
    // sensitive (default: false) (extension: _hatsu.sensitive)
    // attachment
    // context (?)
    // conversation (?)
    // license (default: undefined) (extension: _hatsu.license)
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NoteSource {
    pub content: String,
    pub media_type: String,
}

impl Note {
    pub fn new(
        actor: &ApubUser,
        json: JsonUserFeedItem,
        data: &Data<AppData>,
    ) -> Result<Self, AppError> {
        // TODO: match json._hatsu.source (string)
        let mut sources: Vec<Option<String>> = vec![json.title, json.summary];

        // TODO: parse_item_id (check url)
        // https://example.com/foo/bar => https://example.com/foo/bar
        // /foo/bar => https://example.com/foo/bar
        // foo/bar => https://example.com/foo/bar
        let json_id = json
            .url
            .unwrap_or_else(|| Url::parse(&json.id).unwrap())
            .to_string();
        sources.push(Some(json_id));

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

        let id = hatsu_utils::url::generate_object_url(data.domain(), json.id)?.into();

        Ok(Self {
            kind: NoteType::Note,
            id,
            attributed_to: actor.id().into(),
            to: vec![Url::parse(&format!("{}/followers", actor.id()))?],
            cc: vec![public()],
            content,
            source: Some(serde_json::to_value(NoteSource::new(source))?),
            // TODO: remove
            in_reply_to: None,
            // TODO: test this
            tag: json.tags.map(|tags: Vec<String>| {
                tags.iter()
                    .map(|tag| {
                        serde_json::to_value(Hashtag::new(
                            Url::parse(&format!(
                                "https://{}/t/{}",
                                data.domain(),
                                urlencoding::encode(tag),
                            ))
                            .unwrap(),
                            format!("#{tag}"),
                        ))
                        .unwrap()
                    })
                    .collect()
            }),
            published: hatsu_utils::date::now(),
            updated: None,
        })
    }

    #[async_recursion::async_recursion]
    pub async fn check_in_reply_to_root(
        self,
        data: &Data<AppData>,
    ) -> Result<Option<String>, AppError> {
        match self.in_reply_to.map(|url| url.to_string()) {
            Some(in_reply_to) if in_reply_to.starts_with(&format!("https://{}", data.domain())) => {
                Ok(Some(in_reply_to))
            }
            Some(in_reply_to) => match Post::find_by_id(&in_reply_to).one(&data.conn).await? {
                Some(db_post) => {
                    let apub_post: ApubPost = db_post.into();
                    let note = apub_post.into_json(data).await?;

                    Self::check_in_reply_to_root(note, data).await
                }
                None => Ok(None),
            },
            None => Ok(None),
        }
    }
}

impl NoteSource {
    pub fn new(source: String) -> Self {
        Self {
            content: source,
            media_type: "text/markdown".to_string(),
        }
    }
}
