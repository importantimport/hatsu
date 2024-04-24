// https://www.rfc-editor.org/rfc/rfc6415

use activitypub_federation::config::Data;
use axum::{
    debug_handler,
    http::header::{self, HeaderMap, HeaderValue},
    response::Redirect,
    Json,
};
use hatsu_utils::AppData;

use crate::entities::HostMeta;

/// The host-meta Redirect.
#[utoipa::path(
    get,
    tag = "well_known",
    path = "/.well-known/host-meta",
    responses((status = TEMPORARY_REDIRECT)),
)]
#[debug_handler]
pub async fn redirect(
    // TODO: use axum_extra::TypedHeader
    // https://github.com/hyperium/headers/issues/53
    headers: HeaderMap,
) -> Redirect {
    headers.get(header::ACCEPT).map_or_else(
        || Redirect::temporary("/.well-known/host-meta.xml"),
        |accept| match accept.to_str() {
            Ok(accept) if accept.contains("json") =>
                Redirect::temporary("/.well-known/host-meta.json"),
            _ => Redirect::temporary("/.well-known/host-meta.xml"),
        },
    )
}

/// The host-meta Document.
#[utoipa::path(
    get,
    tag = "well_known",
    path = "/.well-known/host-meta.xml",
    responses(
        (status = OK, description = "", body = String),
    ),
)]
#[debug_handler]
pub async fn xml(data: Data<AppData>) -> (HeaderMap, String) {
    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("application/xml+xrd"),
    );
    (
        headers,
        format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
        <XRD xmlns="http://docs.oasis-open.org/ns/xri/xrd-1.0">
            <Link rel="lrdd" type="application/json" template="https://{}/.well-known/webfinger?resource={{uri}}"></Link>
        </XRD>"#,
            data.domain()
        ),
    )
}

/// The host-meta.json Document.
#[utoipa::path(
    get,
    tag = "well_known",
    path = "/.well-known/host-meta.json",
    responses(
        (status = OK, description = "", body = HostMeta),
    ),
)]
#[debug_handler]
pub async fn json(data: Data<AppData>) -> Json<HostMeta> {
    Json(HostMeta::new(&data))
}
