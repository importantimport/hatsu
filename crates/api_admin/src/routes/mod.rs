use activitypub_federation::config::Data;
use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::{self, Next},
    response::Response,
};
use hatsu_utils::AppData;
use utoipa::OpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::entities::{CreateRemoveAccount, CreateRemoveAccountResult};

mod create_account;
mod remove_account;

#[derive(OpenApi)]
#[openapi(components(schemas(CreateRemoveAccount, CreateRemoveAccountResult)))]
pub struct HatsuAdminApi;

pub fn routes() -> OpenApiRouter {
    OpenApiRouter::with_openapi(HatsuAdminApi::openapi())
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
