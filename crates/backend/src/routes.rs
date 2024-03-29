use axum::{http::Response, response::IntoResponse, routing::get, Router};

// ./hatsu --version
async fn root() -> impl IntoResponse {
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
}
