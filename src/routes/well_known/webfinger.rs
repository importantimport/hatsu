use activitypub_federation::{
    config::Data,
    fetch::webfinger::{build_webfinger_response, extract_webfinger_name, Webfinger},
};
use axum::{
    debug_handler,
    extract::Query,
    Json
};
use hatsu_db_schema::{
    prelude::*,
    user::Model as DbUser,
};
use sea_orm::*;
use serde::Deserialize;
use url::Url;

use crate::{
    AppData,
    AppError,
};

#[derive(Deserialize)]
pub struct WebfingerQuery {
    resource: String,
}

#[debug_handler]
pub async fn webfinger(
    Query(query): Query<WebfingerQuery>,
    data: Data<AppData>,
) -> Result<Json<Webfinger>, AppError> {
    tracing::info!("{}", &query.resource);

    let name = match extract_webfinger_name(&query.resource, &data) {
        Ok(name) => name,
        _ => {
            // extract webfinger domain
            // acct:any@example.com (extract example.com)
            let vec: Vec<&str> = query.resource.split('@').collect();
            vec[1].to_string()
            // TODO:
            // match vec.get(1) {
            //     Some(domain) => domain,
            //     None => AppError::NotFound {
            //         kind: "User".to_string(),
            //         name: query.resource,
            //     }
            // }
        }
    };
    let id = format!("https://{}/u/{}", data.domain(), &name);

    let db_user: Option<DbUser> = User::find_by_id(&id)
        .one(&data.conn)
        .await?;

    Ok(Json(build_webfinger_response(
        query.resource,
        Url::parse(&db_user.unwrap().id)?
    )))
}
