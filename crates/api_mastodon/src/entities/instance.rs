use activitypub_federation::config::Data;
use hatsu_utils::{AppData, AppError};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use url::Url;
use utoipa::ToSchema;

use super::Account;

/// <https://docs.joinmastodon.org/entities/Instance/>
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct Instance {
    domain: String,
    title: String,
    version: String,
    source_url: Url,
    description: String,
    usage: Value,
    thumbnail: Value,
    languages: Vec<String>,
    configuration: Value,
    registrations: Value,
    contact: InstanceContact,
    rules: Vec<Value>,
}

impl Instance {
    pub async fn new(data: &Data<AppData>) -> Result<Self, AppError> {
        Ok(Self {
            domain: data.domain().to_string(),
            title: data
                .env
                .hatsu_node_name
                .clone()
                .unwrap_or_else(|| String::from("Hatsu")),
            version: String::from(env!("CARGO_PKG_VERSION")),
            source_url: Url::parse("https://github.com/importantimport/hatsu")?,
            description: data
                .env
                .hatsu_node_description
                .clone()
                .unwrap_or_else(|| String::from(env!("CARGO_PKG_DESCRIPTION"))),
            usage: json!({
                "users": {
                    "active_month": 0
                }
            }),
            thumbnail: json!({
                "url": format!("https://{}/favicon.svg", data.domain()),
            }),
            languages: vec![],
            configuration: json!({}),
            registrations: json!({
                "enabled": false,
                "approval_required": false,
            }),
            contact: InstanceContact::new(data).await?,
            rules: vec![],
        })
    }
}

/// <https://docs.joinmastodon.org/entities/Instance/#contact>
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct InstanceContact {
    // TODO: env HATSU_CONTACT_EMAIL
    email: String,
    account: Account,
}

impl InstanceContact {
    pub async fn new(data: &Data<AppData>) -> Result<Self, AppError> {
        Ok(Self {
            email: String::new(),
            account: Account::primary_account(data).await?,
        })
    }
}

/// <https://docs.joinmastodon.org/entities/V1_Instance/>
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct InstanceV1 {
    uri: String,
    title: String,
    short_description: String,
    description: String,
    email: String,
    version: String,
    urls: Value,
    stats: Value,
    thumbnail: Url,
    languages: Vec<String>,
    registrations: bool,
    approval_required: bool,
    invites_enabled: bool,
    configuration: Value,
    contact_account: Account,
    rules: Vec<Value>,
}

impl InstanceV1 {
    pub fn from_instance(instance: Instance) -> Result<Self, AppError> {
        Ok(Self {
            uri: instance.domain.clone(),
            title: instance.title,
            short_description: instance.description.clone(),
            description: instance.description,
            // TODO: env HATSU_CONTACT_EMAIL
            email: String::new(),
            version: instance.version,
            urls: json!({}),
            stats: json!({}),
            thumbnail: Url::parse(&format!("https://{}", instance.domain))?.join("/favicon.svg")?,
            languages: instance.languages,
            registrations: false,
            approval_required: false,
            invites_enabled: true,
            configuration: instance.configuration,
            contact_account: instance.contact.account,
            rules: instance.rules,
        })
    }
}
