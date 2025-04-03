use axum::{Json, Router, routing::get};
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_scalar::{Scalar, Servable};

use crate::{favicon, openapi::ApiDoc};

pub fn routes() -> Router {
    let (api_router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .merge(hatsu_api::routes())
        .merge(hatsu_api_admin::routes())
        .merge(hatsu_api_apub::routes())
        .merge(hatsu_api_mastodon::routes())
        .merge(hatsu_nodeinfo::routes())
        .merge(hatsu_well_known::routes())
        .split_for_parts();

    let openapi_json = api.clone();

    let api_router = api_router
        .route("/openapi.json", get(|| async move { Json(openapi_json) }))
        .merge(Scalar::with_url("/scalar", api));

    let router = Router::new()
        .route("/favicon.ico", get(favicon::ico))
        .route("/favicon.svg", get(favicon::svg))
        .merge(hatsu_frontend::routes());

    router.merge(api_router)
}
