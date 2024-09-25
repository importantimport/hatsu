use hatsu_utils::AppError;
use serde_json::Value;

use super::ApubActivity;

impl ApubActivity {
    // 转换为 JSON
    pub fn into_json(self) -> Result<Value, AppError> {
        Ok(serde_json::from_value(self.activity.clone())?)
    }
}
