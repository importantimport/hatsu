use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify,
    OpenApi,
};

#[derive(OpenApi)]
#[openapi(
    info(title = "Hatsu"),
    paths(
        hatsu_api::routes::generate_204::generate_204,
        hatsu_api_admin::routes::create_account::create_account,
        hatsu_api_admin::routes::remove_account::remove_account,
        hatsu_api_apub::activities::activity::activity,
        hatsu_api_apub::posts::post::post,
        hatsu_api_apub::users::user::user,
        hatsu_api_mastodon::routes::instance::instance::v2,
        hatsu_api_mastodon::routes::instance::instance::v1,
        hatsu_api_mastodon::routes::statuses::status_context::status_context,
        hatsu_api_mastodon::routes::statuses::status_favourited_by::status_favourited_by,
        hatsu_api_mastodon::routes::statuses::status_reblogged_by::status_reblogged_by,
        hatsu_nodeinfo::handler::v2_1,
        hatsu_nodeinfo::handler::v2_0,
        hatsu_well_known::routes::host_meta::redirect,
        hatsu_well_known::routes::host_meta::xml,
        hatsu_well_known::routes::host_meta::json,
        hatsu_well_known::routes::nodeinfo::discovery,
        hatsu_well_known::routes::webfinger::webfinger,
    ),
    components(
        schemas(
            hatsu_utils::AppError,
            hatsu_api_admin::entities::CreateRemoveAccount,
            hatsu_api_admin::entities::CreateRemoveAccountResult,
            hatsu_api_mastodon::entities::Account,
            hatsu_api_mastodon::entities::Context,
            hatsu_api_mastodon::entities::CustomEmoji,
            hatsu_api_mastodon::entities::Instance,
            hatsu_api_mastodon::entities::InstanceContact,
            hatsu_api_mastodon::entities::InstanceV1,
            hatsu_api_mastodon::entities::Status,
            hatsu_apub::actors::User,
            hatsu_apub::actors::UserAttachment,
            hatsu_apub::actors::UserImage,
            hatsu_apub::actors::PublicKeySchema,
            hatsu_apub::links::Tag,
            hatsu_apub::links::Emoji,
            hatsu_apub::links::EmojiIcon,
            hatsu_apub::links::Hashtag,
            hatsu_apub::links::Mention,
            hatsu_apub::objects::Note,
            hatsu_nodeinfo::schema::NodeInfo,
            hatsu_nodeinfo::schema::NodeInfoSoftware,
            hatsu_nodeinfo::schema::NodeInfoServices,
            hatsu_nodeinfo::schema::NodeInfoUsage,
            hatsu_nodeinfo::schema::NodeInfoUsers,
            hatsu_nodeinfo::schema::NodeInfoMetadata,
            hatsu_well_known::entities::HostMeta,
            hatsu_well_known::entities::HostMetaLink,
            hatsu_well_known::entities::NodeInfoWellKnown,
            hatsu_well_known::entities::NodeInfoWellKnownLink,
            hatsu_well_known::entities::WebfingerSchema,
            hatsu_well_known::entities::WebfingerSchemaLink,
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
