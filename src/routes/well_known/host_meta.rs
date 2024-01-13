// https://www.rfc-editor.org/rfc/rfc6415

use activitypub_federation::config::Data;
use axum::{
    body::Body,
    http::{header::ACCEPT, HeaderMap, HeaderValue, Response, StatusCode},
    response::{IntoResponse, Redirect},
    Json,
};
use hatsu_utils::AppData;
use serde::Serialize;

#[allow(clippy::module_name_repetitions)]
#[derive(Serialize)]
pub struct HostMetaJson {
    links: Vec<Link>,
}

#[derive(Serialize)]
struct Link {
    rel: String,
    #[serde(rename = "type")]
    kind: String,
    template: String,
}

pub async fn host_meta(
    // TODO: use axum_extra::TypedHeader
    // https://github.com/hyperium/headers/issues/53
    headers: HeaderMap,
) -> impl IntoResponse {
    match headers.get(ACCEPT) {
        Some(accept) => match accept.to_str() {
            Ok(accept) if accept.contains("application/jrd+json") => {
                Redirect::temporary("/.well-known/host-meta.json")
            }
            Ok(accept) if accept.contains("application/xrd+xml") => {
                Redirect::temporary("/.well-known/host-meta.xrd")
            }
            Ok(accept) if accept.contains("application/json") => {
                Redirect::temporary("/.well-known/host-meta.json")
            }
            Ok(accept) if accept.contains("application/xml") => {
                Redirect::temporary("/.well-known/host-meta.xrd")
            }
            _ => Redirect::temporary("/.well-known/host-meta.xrd"),
        },
        None => Redirect::temporary("/.well-known/host-meta.xrd"),
    }
}

// .well-known/host-meta.xrd
// .well-known/host-meta.xml
#[allow(clippy::module_name_repetitions)]
pub async fn host_meta_xrd(data: Data<AppData>) -> impl IntoResponse {
    let host_meta = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
        <XRD xmlns="http://docs.oasis-open.org/ns/xri/xrd-1.0">
            <Link rel="lrdd" type="application/json" template="https://{}/.well-known/webfinger?resource={{uri}}"></Link>
        </XRD>"#,
        data.domain()
    );
    let mut headers = HeaderMap::new();
    headers.insert(
        "Content-Type",
        HeaderValue::from_static("application/xml+xrd"),
    );
    (headers, Response::new(Body::from(host_meta)))
}

// .well-known/host-meta.jrd
// .well-known/host-meta.json
// TODO: application/jrd+json
#[allow(clippy::module_name_repetitions)]
pub async fn host_meta_json(data: Data<AppData>) -> impl IntoResponse {
    let host_meta_json = HostMetaJson {
        links: vec![Link {
            rel: "lrdd".to_string(),
            kind: "application/json".to_string(),
            template: format!(
                "https://{}/.well-known/webfinger?resource={{uri}}",
                data.domain()
            ),
        }],
    };

    (StatusCode::OK, Json(host_meta_json))
}
