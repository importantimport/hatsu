#![allow(clippy::needless_for_each)]

use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(info(title = "Hatsu"), components(schemas(hatsu_utils::AppError)))]
pub struct ApiDoc;
