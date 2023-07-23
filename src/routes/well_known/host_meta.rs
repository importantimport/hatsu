// https://www.rfc-editor.org/rfc/rfc6415

use activitypub_federation::config::Data;
use axum::{
    Json,
    body::Body,
    http::{HeaderMap, Response},
    response::IntoResponse,
};
use serde::Serialize;

#[derive(Serialize)]
pub struct HostMetaJson {
    links: Vec<Link>,
}

#[derive(Serialize)]
struct Link {
    rel: String,
    href: String,
}

use crate::AppData;

/// TODO: detect 'Accept' header
pub async fn host_meta(
    data: Data<AppData>,
) -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/xml+xrd".parse().unwrap());

    let host_meta = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
        <XRD xmlns="http://docs.oasis-open.org/ns/xri/xrd-1.0">
        <Link rel="lrdd" type="application/xrd+xml" template="https://{}/.well-known/webfinger?resource={{uri}}">
        </Link>
        </XRD>"#,
        data.domain()
    );

    (headers, Response::new(Body::from(host_meta)))
}

pub async fn host_meta_json(
    data: Data<AppData>
) -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());

    let host_meta_json = HostMetaJson {
        links: vec![
                Link {
                    rel: "lrdd".to_string(),
                    href: format!("https://{}/.well-known/webfinger?resource={{uri}}", data.domain()),
                }
            ]
    };

    (headers, Json(host_meta_json))
}
