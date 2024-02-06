use activitypub_federation::config::Data;
use axum::{http::StatusCode, response::IntoResponse, Json};
use hatsu_utils::AppData;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct NodeInfoWellKnown {
    links: Vec<NodeInfoWellKnownLink>,
}

#[derive(Debug, Deserialize, Serialize)]
struct NodeInfoWellKnownLink {
    rel: String,
    href: String,
}

pub async fn nodeinfo(data: Data<AppData>) -> impl IntoResponse {
    let nodeinfo = NodeInfoWellKnown {
        links: vec![
            NodeInfoWellKnownLink {
                rel: String::from("http://nodeinfo.diaspora.software/ns/schema/2.0"),
                href: format!("https://{}/nodeinfo/2.0.json", data.domain()),
            },
            NodeInfoWellKnownLink {
                rel: String::from("http://nodeinfo.diaspora.software/ns/schema/2.1"),
                href: format!("https://{}/nodeinfo/2.1.json", data.domain()),
            },
        ],
    };

    (StatusCode::OK, Json(nodeinfo))
}
