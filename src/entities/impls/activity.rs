use serde_json::{from_str, Value};

use crate::{
    AppError,
    entities::activity::Model as DbActivity,
};

impl DbActivity {
    // 转换为 JSON
    pub fn into_json(self) -> Result<Value, AppError> {
        Ok(from_str(&self.activity)?)
    }
}
