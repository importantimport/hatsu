use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;

use crate::entities::{
    Account,
    Context,
    CustomEmoji,
    Instance,
    InstanceContact,
    InstanceV1,
    Status,
};

mod instance;
mod statuses;

pub const TAG: &str = "mastodon";

#[derive(OpenApi)]
#[openapi(
    components(schemas(
        Account,
        Context,
        CustomEmoji,
        Instance,
        InstanceContact,
        InstanceV1,
        Status
    )),
    tags((name = TAG, description = "Mastodon Compatible API (/api/v{1,2}/)"))
)]
pub struct MastodonApi;

pub fn routes() -> OpenApiRouter {
    OpenApiRouter::with_openapi(MastodonApi::openapi())
        .merge(instance::routes())
        .merge(statuses::routes())
}
