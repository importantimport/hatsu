use axum::{
    debug_handler,
    http::header::{self, HeaderMap, HeaderValue},
};

#[debug_handler]
pub async fn ico() -> (HeaderMap, Vec<u8>) {
    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
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
