// https://github.com/LemmyNet/activitypub-federation-rust/blob/61085a643f05dbb70502b3c519fd666214b7e308/examples/live_federation/objects/post.rs
// https://github.com/LemmyNet/lemmy/blob/main/crates/apub/assets

use activitypub_federation::{
    config::Data,
    fetch::object_id::ObjectId,
    kinds::{public, object::NoteType},
    protocol::helpers::deserialize_one_or_many,
    traits::Actor,
};
use chrono::{Local, SecondsFormat};
use hatsu_utils::{
    AppData,
    AppError,
    markdown::markdown_to_html,
};
use serde::{Deserialize, Serialize};
use url::Url;
use urlencoding::encode;

use crate::{
    actors::{ApubUser, JsonUserFeedItem},
    links::Hashtag,
    objects::ApubPost,
};

/// https://www.w3.org/ns/activitystreams#Note
#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Note {
    #[serde(rename = "type")]
    pub kind: NoteType,
    pub id: ObjectId<ApubPost>,
    pub attributed_to: ObjectId<ApubUser>,
    #[serde(deserialize_with = "deserialize_one_or_many")]
    pub to: Vec<Url>,
    #[serde(deserialize_with = "deserialize_one_or_many")]
    pub cc: Vec<Url>,
    pub content: String,
    /// TODO: customization via item._hatsu.source
    pub source: NoteSource,
    pub in_reply_to: Option<ObjectId<ApubPost>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Vec<Hashtag>>,
    pub published: Option<String>,
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
    pub fn new(note_id: String, actor: &ApubUser, source: String) -> Result<Self, AppError> {
        Ok(Self {
            kind: Default::default(),
            id: Url::parse(&note_id)?.into(),
            attributed_to: actor.id().into(),
            to: vec![public()],
            cc: vec![Url::parse(&format!("{}/followers", actor.id()))?],
            content: markdown_to_html(&source),
            source: NoteSource::new(source),
            in_reply_to: None,
            tag: None,
            published: Some(Local::now().to_rfc3339_opts(SecondsFormat::Secs, true)),
            updated: None,
        })
    }

    // TODO: replace Note::new()
    pub fn new_default(actor: &ApubUser, json: JsonUserFeedItem, data: &Data<AppData>) -> Result<Self, AppError> {
        // TODO: match json._hatsu.source (string)
        let mut sources: Vec<Option<String>> = vec![json.title, json.summary];

        // TODO: parse_item_id (check url)
        // https://example.com/foo/bar => https://example.com/foo/bar
        // /foo/bar => https://example.com/foo/bar 
        // foo/bar => https://example.com/foo/bar
        let json_id = json.url.unwrap_or_else(|| Url::parse(&json.id).unwrap()).to_string();
        sources.push(Some(json_id));

        let mut source = sources
            .iter()
            .filter_map(|source| source.clone())
            .collect::<Vec<String>>()
            .join("\n\n");

        let mut content = markdown_to_html(&source);

        // TODO: json._hatsu.tags (Option<false>)
        if json.tags.is_some() {
            source.push_str(&format!(
                "\n\n{}",
                json.tags
                    .clone()
                    .unwrap()
                    .iter()
                    .map(|tag| "#".to_owned() + tag)
                    .collect::<Vec<String>>()
                    .join(" ")
            ));

            content.push_str(&format!(
                "\n\n{}",
                json.tags
                    .clone()
                    .unwrap()
                    .iter()
                    // TODO: test urlencoding::encode()
                    .map(|tag| format!("<a href=\"https://{}/t/{}\" rel=\"tag\">#<span>{}</span></a>", data.domain(), encode(tag), tag))
                    .collect::<Vec<String>>()
                    .join(" ")
            ));
        }

        let id = Url::parse(&format!("https://{}/o/{}", data.domain(), json.id))?.into();

        Ok(Self {
            kind: Default::default(),
            id,
            attributed_to: actor.id().into(),
            to: vec![public()],
            cc: vec![Url::parse(&format!("{}/followers", actor.id()))?],
            content,
            source: NoteSource::new(source),
            // TODO: remove
            in_reply_to: None,
            // TODO: test this
            tag: json.tags.map(|tags: Vec<String>| tags
                .iter()
                .map(|tag| Hashtag {
                    kind: Default::default(),
                    href: Url::parse(&format!("https://{}/t/{}", data.domain(), encode(tag))).unwrap(),
                    name: "#".to_owned() + tag,
                })
                .collect()),
            published: Some(Local::now().to_rfc3339_opts(SecondsFormat::Secs, true)),
            updated: None,
        })
    }
}

impl NoteSource {
    pub fn new(source: String) -> Self {
        Self {
            content: source,
            media_type: "text/markdown".to_string()
        }
    }
}
