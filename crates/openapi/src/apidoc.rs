use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify,
    OpenApi,
};

#[derive(OpenApi)]
#[openapi(
    paths(
        hatsu_api_admin::routes::create_account::create_account,
        hatsu_api_admin::routes::remove_account::remove_account,
        hatsu_api_apub::activities::activity::handler,
        hatsu_api_apub::posts::post::handler,
        hatsu_api_apub::users::user::handler,
        hatsu_api_mastodon::routes::statuses::status_context::status_context,
        hatsu_api_mastodon::routes::statuses::status_favourited_by::status_favourited_by,
        hatsu_api_mastodon::routes::statuses::status_reblogged_by::status_reblogged_by,
    ),
    components(
        schemas(
            hatsu_utils::AppError,
            hatsu_api_admin::entities::CreateRemoveAccount,
            hatsu_api_admin::entities::CreateRemoveAccountResult,
            hatsu_api_mastodon::entities::Account,
            hatsu_api_mastodon::entities::Context,
            hatsu_api_mastodon::entities::CustomEmoji,
            hatsu_api_mastodon::entities::Status,
            hatsu_apub::actors::Service,
            hatsu_apub::actors::ServiceImage,
            hatsu_apub::actors::ServiceAttachment,
            hatsu_apub::actors::PublicKeySchema,
            hatsu_apub::objects::Note,
        )
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = "hatsu", description = "Hatsu API (/api/v0/)"),
        (name = "hatsu::admin", description = "Hatsu Admin API (/api/v0/admin/)"),
        (name = "apub", description = "ActivityPub API"),
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
                SecurityScheme::ApiKey(ApiKey::Query(ApiKeyValue::new("token"))),
            );
        }
    }
}
