use activitypub_federation::{
    config::Data,
    fetch::webfinger::{build_webfinger_response, extract_webfinger_name, Webfinger},
};
use axum::{extract::Query, Json};
use serde::Deserialize;
use url::Url;

use crate::error::Error;

#[derive(Deserialize)]
pub struct WebfingerQuery {
    resource: String,
}

pub async fn webfinger(
    Query(query): Query<WebfingerQuery>,
    data: Data<String>, // ) -> impl IntoResponse {
) -> Result<Json<Webfinger>, Error> {
    tracing::info!("{}", &query.resource);

    let name = extract_webfinger_name(&query.resource, &data)?;

    tracing::info!("{}", name);
    // let user = data.read_user(&name)?;

    Ok(Json(build_webfinger_response(query.resource, Url::parse("https://www.google.com").unwrap())))
}
