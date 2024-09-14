use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify,
    OpenApi,
};

#[derive(OpenApi)]
#[openapi(
    info(title = "Hatsu"),
    paths(
        hatsu_api_apub::posts::post::post,
    ),
    components(
        schemas(
            hatsu_utils::AppError,
        )
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = "hatsu", description = "Hatsu API (/api/v0/)"),
        (name = "hatsu::admin", description = "Hatsu Admin API (/api/v0/admin/)"),
        (name = "apub", description = "ActivityPub API"),
        (name = "nodeinfo", description = "NodeInfo (/nodeinfo/)"),
        (name = "mastodon", description = "Mastodon Compatible API (/api/v{1,2}/)"),
        (name = "well_known", description = "Well Known (/.well-known/)"),
    )
)]
pub struct ApiDoc;

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
