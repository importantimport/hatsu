use activitypub_federation::fetch::object_id::ObjectId;
use url::Url;
use uuid::Uuid;

use crate::{
    AppError,
    entities::activity::Model as DbActivity,
};

/// 创建一个 Activity URL
/// 
/// Example: https://hatsu.local/a/80075c4e-a4e1-4b29-8b89-417cf7339be9
pub fn generate_activity_id(domain: &str, id: Option<String>) -> Result<ObjectId<DbActivity>, AppError> {
    Ok(Url::parse(&format!(
        "https://{}/a/{}",
        domain,
        id.unwrap_or_else(|| Uuid::new_v4().to_string())
    ))?.into())
}
