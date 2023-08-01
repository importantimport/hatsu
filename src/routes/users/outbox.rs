use activitypub_federation::{
  axum::json::FederationJson,
  config::Data, protocol::context::WithContext,
};
use axum::{
  debug_handler,
  extract::Path,
};
use url::Url;

use crate::{
  AppData,
  error::AppError,
  protocol::collections::outbox::Outbox,
};

#[debug_handler]
pub async fn user_outbox(
  Path(name): Path<String>,
  data: Data<AppData>,
) -> Result<FederationJson<WithContext<Outbox>>, AppError> {
  Ok(FederationJson(WithContext::new_default(
    Outbox::new(
      Url::parse(&format!("https://{}/u/{}/outbox", data.domain(), name))?
    )?
  )))
}
