use utoipa::OpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};

mod generate_204;

pub const TAG: &str = "hatsu";

#[derive(OpenApi)]
#[openapi(tags((name = TAG, description = "Hatsu API (/api/v0/)")))]
pub struct HatsuApi;

pub fn routes() -> OpenApiRouter {
    OpenApiRouter::with_openapi(HatsuApi::openapi()).routes(routes!(generate_204::generate_204))
}
