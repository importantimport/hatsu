use activitypub_federation::{
    axum::json::FederationJson,
    config::Data, protocol::context::WithContext,
};
use axum::{
    debug_handler,
    extract::Path,
    response::{IntoResponse, Redirect},
};
use url::Url;

use crate::{
    AppData,
    AppError,
    protocol::collections::outbox::Outbox,
};

#[debug_handler]
pub async fn handler(
    Path(name): Path<String>,
    data: Data<AppData>,
) -> Result<FederationJson<WithContext<Outbox>>, AppError> {
    Ok(FederationJson(WithContext::new_default(
        Outbox::new(
            Url::parse(&format!("https://{}/u/{}/outbox", data.domain(), name))?
        )?
    )))
}

#[debug_handler]
pub async fn redirect(Path(name): Path<String>) -> impl IntoResponse {
    Redirect::permanent(&format!("/u/{}/outbox", name)).into_response()
}
