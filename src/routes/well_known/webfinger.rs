use activitypub_federation::{
    config::Data,
    fetch::webfinger::{build_webfinger_response, extract_webfinger_name, Webfinger},
};
use axum::{extract::Query, Json};
use sea_orm::*;
use serde::Deserialize;
use url::Url;

use crate::{
    entities::{prelude::*, *},
    error::Error,
    AppData
};

#[derive(Deserialize)]
pub struct WebfingerQuery {
    resource: String,
}

pub async fn webfinger(
    Query(query): Query<WebfingerQuery>,
    data: Data<AppData>,
) -> Result<Json<Webfinger>, Error> {
    tracing::info!("{}", &query.resource);

    let name = extract_webfinger_name(&query.resource, &data)?;

    let _user: Option<user::Model> = User::find()
        .filter(
            Condition::all()
                .add(user::Column::Name.eq(name))
                .add(user::Column::Local.eq(true))
        )
        .one(&data.conn)
        .await?;

    Ok(Json(build_webfinger_response(query.resource, Url::parse("https://www.google.com").unwrap())))
}
