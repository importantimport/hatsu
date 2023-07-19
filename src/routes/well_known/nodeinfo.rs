use axum::{
    response::IntoResponse,
    http::StatusCode,
    Json
};
use serde::Serialize;

#[derive(Serialize)]
pub struct Nodeinfo {
    links: Vec<Link>,
}

#[derive(Serialize)]
struct Link {
    rel: String,
    href: String,
}

pub async fn nodeinfo() -> impl IntoResponse {
    let nodeinfo = Nodeinfo {
        links: vec![
            Link {
                rel: "http://nodeinfo.diaspora.software/ns/schema/2.1".to_string(),
                href: "https://hatsu.local/nodeinfo/2.1.json".to_string(),
            }
        ]
    };

    (StatusCode::OK, Json(nodeinfo))
}
