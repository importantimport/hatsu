use activitypub_federation::config::Data;
use axum::{
    response::IntoResponse,
    http::StatusCode,
    Json
};
use serde::{Deserialize, Serialize};

use crate::AppData;

#[derive(Debug, Deserialize, Serialize)]
pub struct NodeInfoWellKnown {
    links: Vec<NodeInfoWellKnownLink>,
}

#[derive(Debug, Deserialize, Serialize)]
struct NodeInfoWellKnownLink {
    rel: String,
    href: String,
}

pub async fn nodeinfo(
    data: Data<AppData>
) -> impl IntoResponse {
    let nodeinfo = NodeInfoWellKnown {
        links: vec![
            NodeInfoWellKnownLink {
                rel: "http://nodeinfo.diaspora.software/ns/schema/2.0".to_string(),
                href: format!("https://{}/nodeinfo/2.0.json", data.domain()),
            },
            NodeInfoWellKnownLink {
                rel: "http://nodeinfo.diaspora.software/ns/schema/2.1".to_string(),
                href: format!("https://{}/nodeinfo/2.1.json", data.domain()),
            }
        ]
    };

    (StatusCode::OK, Json(nodeinfo))
}
