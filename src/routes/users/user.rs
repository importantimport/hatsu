use activitypub_federation::{
  axum::json::FederationJson,
  config::Data,
  protocol::context::WithContext,
  traits::Object,
};
use axum::{
  debug_handler,
  extract::Path,
};
use sea_orm::*;

use crate::{
  AppData,
  entities::{
    prelude::*,
    user::Model as DbUser,
  },
  error::Error,
  objects::user::Person
};

#[debug_handler]
pub async fn user(
    Path(name): Path<String>,
    data: Data<AppData>,
) -> Result<FederationJson<WithContext<Person>>, Error> {
    let id = format!("https://{}/{}", data.domain(), &name);
    let db_user: Option<DbUser> = User::find_by_id(&id)
        .one(&data.conn)
        .await?;
    let json_user = db_user.unwrap().into_json(&data).await?;

    Ok(FederationJson(WithContext::new_default(json_user)))
}
