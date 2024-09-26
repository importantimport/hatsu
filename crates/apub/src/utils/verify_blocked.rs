use activitypub_federation::config::Data;
use axum::http::StatusCode;
use hatsu_db_schema::prelude::BlockedUrl;
use hatsu_utils::{AppData, AppError};
use sea_orm::EntityTrait;
use url::Url;

pub async fn verify_blocked(url: &Url, data: &Data<AppData>) -> Result<(), AppError> {
    let blocked_url = BlockedUrl::find().all(&data.conn).await?;

    if blocked_url
        .clone()
        .into_iter()
        .filter(|url| url.is_instance)
        .filter_map(|url| Url::parse(&url.id).ok())
        .map(|url| url.origin())
        .any(|instance| url.origin().eq(&instance))
    {
        Err(AppError::new(
            format!("blocked instance: {:?}", url.host_str()),
            None,
            Some(StatusCode::BAD_REQUEST),
        ))
    } else if blocked_url
        .into_iter()
        .filter(|url| !url.is_instance)
        .filter_map(|url| Url::parse(&url.id).ok())
        .any(|actor| url.eq(&actor))
    {
        Err(AppError::new(
            format!("blocked actor: {}", url),
            None,
            Some(StatusCode::BAD_REQUEST),
        ))
    } else {
        Ok(())
    }
}
