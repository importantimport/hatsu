use axum::{http::Response, routing::get, Router};

use crate::favicon;

// ./hatsu --version
async fn root() -> Response<String> {
    let version = env!("CARGO_PKG_VERSION");
    let codename = "01_ballade";

    Response::new(format!("Hatsu v{version} \"{codename}\""))
}

pub fn routes() -> Router {
    Router::new()
        .merge(hatsu_api_admin::routes())
        .merge(hatsu_api_apub::routes())
        .merge(hatsu_api_mastodon::routes())
        .merge(hatsu_nodeinfo::routes())
        .merge(hatsu_openapi::routes())
        .merge(hatsu_well_known::routes())
        .route("/", get(root))
        .route("/favicon.ico", get(favicon::ico))
        .route("/favicon.svg", get(favicon::svg))
}
