// https://www.rfc-editor.org/rfc/rfc6415

use activitypub_federation::config::Data;
use hatsu_utils::AppData;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct HostMeta {
    links: Vec<HostMetaLink>,
}

impl HostMeta {
    pub fn new(data: &Data<AppData>) -> Self {
        Self {
            links: vec![HostMetaLink::new(data)],
        }
    }
}

#[derive(Serialize, ToSchema)]
pub struct HostMetaLink {
    rel: String,
    #[serde(rename = "type")]
    kind: String,
    template: String,
}

impl HostMetaLink {
    pub fn new(data: &Data<AppData>) -> Self {
        Self {
            rel: String::from("lrdd"),
            kind: String::from("application/json"),
            template: format!(
                "https://{}/.well-known/webfinger?resource={{uri}}",
                data.domain()
            ),
        }
    }
}
