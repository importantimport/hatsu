use std::str;

use url::Url;
use uuid::Uuid;

use crate::AppError;

pub fn absolutize_relative_url(relative_url: &str, domain: &str) -> Result<Url, AppError> {
    if relative_url.starts_with("https://") {
        Ok(Url::parse(relative_url)?)
    } else {
        Ok(Url::parse(&format!("https://{domain}"))?.join(relative_url)?)
    }
}

/// 创建一个 Activity URL
///
/// Example: <https://hatsu.local/activities/80075c4e-a4e1-4b29-8b89-417cf7339be9>
pub fn generate_activity_url(domain: &str, id: Option<String>) -> Result<Url, AppError> {
    Ok(Url::parse(&format!(
        "https://{}/activities/{}",
        domain,
        id.unwrap_or_else(|| Uuid::now_v7().to_string())
    ))?)
}

/// 创建一个 Post URL
///
/// Example: <https://hatsu.local/post/https://example.com/foo/bar>
pub fn generate_post_url(domain: &str, id: String) -> Result<Url, AppError> {
    match id {
        id if id.starts_with("https://") =>
            Ok(Url::parse(&format!("https://{domain}/posts/{id}",))?),
        _ => Err(AppError::new(
            format!("Invalid Post ID: {id}"),
            serde_json::from_str("Post ID need to starts with https://")?,
            None,
        )),
    }
}

/// 创建一个 User URL
///
/// Example: <https://hatsu.local/user/example.com>
pub fn generate_user_url(domain: &str, id: &str) -> Result<Url, AppError> {
    match id {
        id if !id.starts_with("https://") =>
            Ok(Url::parse(&format!("https://{domain}/users/{id}",))?),
        _ => Err(AppError::new(
            format!("Invalid User ID: {id}"),
            serde_json::from_str("User ID cannot starts with https://")?,
            None,
        )),
    }
}

// pub fn remove_https(url: String) -> String {
//     if str::starts_with(&url, "https://") {
//         let url_without_https = url.trim_start_matches("https://").to_string();
//         url_without_https
//     } else {
//         url
//     }
// }
