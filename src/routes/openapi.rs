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
        super::api::hatsu::v0::admin::create_account::create_account
    ),
    components(
        schemas(
            crate::AppError,
            super::api::hatsu::v0::admin::create_account::CreateAccount,
            super::api::hatsu::v0::admin::create_account::CreateAccountResult,
        )
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = "hatsu", description = "Hatsu API"),
        (name = "hatsu::admin", description = "Hatsu Admin API"),
        (name = "mastodon", description = "Mastodon Compatible API"),
        (name = "activitypub", description = "ActivityPub API"),
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
