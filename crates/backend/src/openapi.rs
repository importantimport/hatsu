use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    info(title = "Hatsu"),
    paths(hatsu_api_apub::posts::post::post),
    components(schemas(hatsu_utils::AppError))
)]
pub struct ApiDoc;
