use std::str;

use url::Url;
use uuid::Uuid;

use crate::AppError;

pub fn absolutize_relative_url(relative_url: String, domain: String) -> Result<Url, AppError> {
    if str::starts_with(&relative_url, "https://") {
        Ok(Url::parse(&relative_url)?)
    } else {
        let origin = Url::parse(&format!("https://{}", domain))?;
        let absolute_url = origin.join(relative_url.as_str())?;
        Ok(absolute_url)
    }
}

/// 创建一个 Activity URL
/// 
/// Example: https://hatsu.local/a/80075c4e-a4e1-4b29-8b89-417cf7339be9
pub fn generate_activity_url(domain: &str, id: Option<String>) -> Result<Url, AppError> {
    Ok(Url::parse(&format!(
        "https://{}/a/{}",
        domain,
        id.unwrap_or_else(|| Uuid::now_v7().to_string())
    ))?)
}

/// 创建一个 Object URL
/// 
/// Example: https://hatsu.local/o/https://example.com/foo/bar
pub fn generate_object_url(domain: &str, id: String) -> Result<Url, AppError> {
    match id {
        id if id.starts_with("https://") => Ok(Url::parse(&format!(
            "https://{}/o/{}",
            domain,
            id,
        ))?),
        _ => Err(AppError::new(format!("Invalid Object ID {}", id), None, None))
    }
}

/// 创建一个 User URL
/// 
/// Example: https://hatsu.local/u/example.com
pub fn generate_user_url(domain: &str, id: &str) -> Result<Url, AppError> {
    match id {
        id if !id.starts_with("https://") => Ok(Url::parse(&format!(
            "https://{}/u/{}",
            domain,
            id,
        ))?),
        _ => Err(AppError::new(format!("Invalid User ID {}", id), None, None))
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
