use utoipa::OpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};

mod generate_204;

#[derive(OpenApi)]
pub struct HatsuApi;

pub fn routes() -> OpenApiRouter {
    OpenApiRouter::with_openapi(HatsuApi::openapi()).routes(routes!(generate_204::generate_204))
}
