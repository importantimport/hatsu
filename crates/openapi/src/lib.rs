use axum::{routing::get, Json, Router};
use utoipa::OpenApi;
use utoipa_scalar::{Scalar, Servable};

mod apidoc;
pub use apidoc::{ApiDoc, SecurityAddon};

pub fn routes() -> Router {
    Router::new()
        .route(
            "/openapi.json",
            get(|| async move { Json(ApiDoc::openapi()) }),
        )
        .merge(Scalar::with_url("/scalar", ApiDoc::openapi()))
}
