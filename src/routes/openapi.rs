use axum::{
    // routing::get,
    // Json,
    Router,
};
use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify,
    OpenApi
};
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(
        super::api::v0::admin::create_account::create_account,
        super::api::v0::admin::remove_account::remove_account,
        super::api::v1::statuses::status_context::status_context,
    ),
    components(
        schemas(
            crate::AppError,
            super::api::v0::admin::create_account::CreateRemoveAccount,
            super::api::v0::admin::create_account::CreateRemoveAccountResult,
            super::api::v1::statuses::status_context::Context,
        )
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = "hatsu", description = "Hatsu API (/api/v0/)"),
        (name = "hatsu::admin", description = "Hatsu Admin API (/api/v0/admin/)"),
        (name = "mastodon", description = "Mastodon Compatible API (/api/v1/)"),
    )
)]
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
        // .route("/openapi.json", get(|| async move { Json(ApiDoc::openapi()) }))
        .merge(SwaggerUi::new("/swagger-ui")
            .url("/openapi.json", ApiDoc::openapi()))
}
