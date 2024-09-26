use activitypub_federation::config::Data;
use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::{self, Next},
    response::Response,
};
use hatsu_utils::AppData;
use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify,
    OpenApi,
};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::entities::{BlockUrlResult, CreateRemoveAccount, CreateRemoveAccountResult};

mod block_url;
mod create_account;
mod remove_account;

pub const TAG: &str = "hatsu::admin";

#[derive(OpenApi)]
#[openapi(
    components(schemas(
        BlockUrlResult,
        CreateRemoveAccount,
        CreateRemoveAccountResult
    )),
    modifiers(&SecurityAddon),
    tags(
        (name = TAG, description = "Hatsu Admin API (/api/v0/admin/)"),
    )
)]
pub struct HatsuAdminApi;

pub struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "api_key",
                SecurityScheme::ApiKey(ApiKey::Query(ApiKeyValue::new("token"))),
            );
        }
    }
}

pub fn routes() -> OpenApiRouter {
    OpenApiRouter::with_openapi(HatsuAdminApi::openapi())
        .routes(routes!(block_url::block_url))
        .routes(routes!(create_account::create_account))
        .routes(routes!(remove_account::remove_account))
        .layer(middleware::from_fn(auth))
}

async fn auth(
    data: Data<AppData>,
    request: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    match &data.env.hatsu_access_token {
        Some(token) => match request.uri().query() {
            Some(query) if query == format!("token={token}") => Ok(next.run(request).await),
            Some(query) if query != format!("token={token}") => Err(StatusCode::UNAUTHORIZED),
            _ => Err(StatusCode::BAD_REQUEST),
        },
        None => Err(StatusCode::UNAUTHORIZED),
    }
}
