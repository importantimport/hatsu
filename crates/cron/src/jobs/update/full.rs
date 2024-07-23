use activitypub_federation::config::FederationConfig;
use apalis::prelude::Data;
use chrono::{DateTime, Utc};
use hatsu_utils::AppData;
use serde::{Deserialize, Serialize};

use crate::tasks;

#[derive(Debug, Deserialize, Serialize)]
pub struct FullUpdate(DateTime<Utc>);

impl From<DateTime<Utc>> for FullUpdate {
    fn from(t: DateTime<Utc>) -> Self {
        Self(t)
    }
}

pub async fn full_update(_job: FullUpdate, data: Data<FederationConfig<AppData>>) -> bool {
    let app_data = data.to_request_data();

    tracing::info!("full update starting...");

    tasks::full_update(&app_data).await.is_ok()
}
