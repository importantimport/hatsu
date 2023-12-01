use axum::{
    routing::get,
    Json,
    Router,
};
use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify,
    OpenApi
};

#[derive(OpenApi)]
#[openapi()]
pub struct ApiDoc;

pub struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "api_key",
                SecurityScheme::ApiKey(
                    ApiKey::Query(
                        ApiKeyValue::new("token")
                    )
                )
            )
        }
    }
}

pub fn handler() -> Router {
    Router::new()
        .route("/openapi.json", get(|| async move { Json(ApiDoc::openapi()) }))
}
