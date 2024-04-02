use activitypub_federation::{
    config::Data,
    fetch::webfinger::{build_webfinger_response, extract_webfinger_name, Webfinger},
};
use axum::{debug_handler, extract::Query, Json};
use hatsu_db_schema::prelude::User;
use hatsu_utils::{AppData, AppError};
use sea_orm::EntityTrait;
use serde::Deserialize;
use url::Url;

#[derive(Deserialize)]
pub struct WebfingerQuery {
    resource: String,
}

/// WebFinger.
#[utoipa::path(
    get,
    tag = "well_known",
    path = "/.well-known/webfinger",
    responses(
        (status = OK, description = "", body = WebfingerSchema),
        (status = NOT_FOUND, description = "", body = AppError),
    ),
)]
#[debug_handler]
pub async fn webfinger(
    Query(query): Query<WebfingerQuery>,
    data: Data<AppData>,
) -> Result<Json<Webfinger>, AppError> {
    tracing::info!("{}", &query.resource);

    let name = if let Ok(name) = extract_webfinger_name(&query.resource, &data) {
        Ok(name)
    } else {
        // extract webfinger domain
        let vec: Vec<&str> = query.resource.split('@').collect();
        match vec.get(1) {
            // acct:example.com@hatsu.local => example.com
            Some(domain) if *domain == data.domain() => Ok(vec[0].trim_start_matches("acct:")),
            // acct:example.com@example.com => example.com
            Some(domain) if Url::parse(&format!("https://{domain}")).is_ok() => Ok(*domain),
            // acct:example.com@unknown => Err
            _ => Err(AppError::not_found("Subject", &query.resource)),
        }
    }?;

    let url = hatsu_utils::url::generate_user_url(data.domain(), name)?;

    match User::find_by_id(&url.to_string()).one(&data.conn).await? {
        // TODO: (optional) http://webfinger.net/rel/avatar
        Some(user) => Ok(Json(build_webfinger_response(
            query.resource,
            Url::parse(&user.id)?,
        ))),
        None => Err(AppError::not_found("Subject", &query.resource)),
    }
}
