use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod apidoc;
pub use apidoc::{ApiDoc, SecurityAddon};

pub fn routes() -> Router {
    Router::new()
        // .route("/openapi.json", get(|| async move { Json(ApiDoc::openapi()) }))
        .merge(SwaggerUi::new("/swagger-ui").url("/openapi.json", ApiDoc::openapi()))
}
