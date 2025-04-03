use activitypub_federation::config::FederationConfig;
use apalis::prelude::Data;
use chrono::{DateTime, Utc};
use hatsu_utils::AppData;
use serde::{Deserialize, Serialize};

use crate::tasks;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct PartialUpdate(DateTime<Utc>);

impl From<DateTime<Utc>> for PartialUpdate {
    fn from(t: DateTime<Utc>) -> Self {
        Self(t)
    }
}

pub async fn partial_update(_job: PartialUpdate, data: Data<FederationConfig<AppData>>) -> bool {
    let app_data = data.to_request_data();
    tasks::partial_update(&app_data).await.is_ok()
}
