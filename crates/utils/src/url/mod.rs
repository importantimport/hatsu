use std::str;

use url::Url;
use uuid::Uuid;

use crate::AppError;

pub fn absolutize_relative_url(url: String, domain: String) -> Result<String, AppError> {
    if str::starts_with(&url, "https://") {
        Ok(url)
    } else {
        let origin = Url::parse(&format!("https://{}", domain))?;
        let absolute_url = origin.join(url.as_str())?.to_string();
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
