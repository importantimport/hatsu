use reqwest::{Client, IntoUrl, Response, header::HeaderValue};

pub async fn get<T: IntoUrl>(url: T) -> reqwest::Result<Response> {
    const USER_AGENT: HeaderValue =
        HeaderValue::from_static(concat!("Hatsu/", env!("CARGO_PKG_VERSION")));
    Client::builder()
        .user_agent(USER_AGENT)
        .build()?
        .get(url)
        .send()
        .await
}
