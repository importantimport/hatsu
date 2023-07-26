// https://www.rfc-editor.org/rfc/rfc6415

use activitypub_federation::{
    config::Data,
    // FEDERATION_CONTENT_TYPE,
};
use axum::{
    Json,
    body::Body,
    http::{
        HeaderMap,
        Response,
        // header::ACCEPT,
        StatusCode
    },
    response::{
        IntoResponse,
        // Redirect
    },
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
/// PR Welcome!
pub async fn host_meta(
    // mut headers: HeaderMap,
    data: Data<AppData>,
) -> impl IntoResponse {

    // let accept: Option<&str> = headers.get(ACCEPT).map(|x| x.to_str().unwrap());

    // tracing::info!("{}", accept.unwrap());

    // if accept == Some(FEDERATION_CONTENT_TYPE) {
    //     Redirect::to(&format!("https://{}/.well-known/host-meta.json", data.domain())).into_response();
    // } else {
        let host_meta = format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
            <XRD xmlns="http://docs.oasis-open.org/ns/xri/xrd-1.0">
            <Link rel="lrdd" type="application/xrd+xml" template="https://{}/.well-known/webfinger?resource={{uri}}">
            </Link>
            </XRD>"#,
            data.domain()
        );
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", "application/xml+xrd".parse().unwrap());
        (headers, Response::new(Body::from(host_meta)))
    // }
}

pub async fn host_meta_json(
    data: Data<AppData>
) -> impl IntoResponse {
    let host_meta_json = HostMetaJson {
        links: vec![
                Link {
                    rel: "lrdd".to_string(),
                    href: format!("https://{}/.well-known/webfinger?resource={{uri}}", data.domain()),
                }
            ]
    };

    (StatusCode::OK, Json(host_meta_json))
}
