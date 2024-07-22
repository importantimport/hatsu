use apalis::prelude::Data;
use chrono::{DateTime, Utc};
use hatsu_utils::{
    AppData,
    // AppError,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PartialUpdate(DateTime<Utc>);

impl From<DateTime<Utc>> for PartialUpdate {
    fn from(value: DateTime<Utc>) -> Self {
        Self(value)
    }
}

// pub async fn partial_update(_job: PartialUpdate, _data: Data<AppData>) -> Result<(), AppError> {
pub async fn partial_update(_job: PartialUpdate, _data: Data<AppData>) {
    tracing::info!("partial update starting...");

    tracing::info!("partial update ok");

    ()
}
