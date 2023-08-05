use url::Url;
use uuid::Uuid;

use crate::AppError;

/// 创建一个 Activity URL
/// 
/// TODO: 使用 UUID v5
/// 
/// Example: https://hatsu.local/a/80075c4e-a4e1-4b29-8b89-417cf7339be9
pub fn generate_activity_id(domain: &str, id: Option<String>) -> Result<Url, AppError> {
    let id = id.unwrap_or_else(|| Uuid::new_v4().to_string());
    Ok(Url::parse(&format!(
        "https://{}/a/{}",
        domain,
        id
    ))?)
}
