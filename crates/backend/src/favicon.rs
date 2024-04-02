use axum::{
    debug_handler,
    http::{HeaderMap, HeaderName, HeaderValue},
};

#[debug_handler]
pub async fn ico() -> (HeaderMap, Vec<u8>) {
    let mut headers = HeaderMap::new();
    headers.insert(
        HeaderName::from_static("Content-Type"),
        HeaderValue::from_static("image/x-icon"),
    );

    (
        headers,
        include_bytes!("../../../assets/favicon.ico").to_vec(),
    )
}

#[debug_handler]
pub async fn svg() -> Vec<u8> {
    include_bytes!("../../../assets/favicon.svg").to_vec()
}
