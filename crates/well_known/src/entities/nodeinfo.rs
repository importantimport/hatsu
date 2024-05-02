use activitypub_federation::config::Data;
use hatsu_utils::AppData;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct NodeInfoWellKnown {
    links: Vec<NodeInfoWellKnownLink>,
}

impl NodeInfoWellKnown {
    pub fn new(data: &Data<AppData>) -> Self {
        Self {
            links: vec![
                NodeInfoWellKnownLink::new(data, "2.1"),
                NodeInfoWellKnownLink::new(data, "2.0"),
            ],
        }
    }
}

#[derive(Serialize, ToSchema)]
pub struct NodeInfoWellKnownLink {
    rel: String,
    href: String,
}

impl NodeInfoWellKnownLink {
    pub fn new(data: &Data<AppData>, version: &str) -> Self {
        Self {
            rel: format!("http://nodeinfo.diaspora.software/ns/schema/{version}"),
            href: format!("https://{}/nodeinfo/{}.json", data.domain(), version),
        }
    }
}
