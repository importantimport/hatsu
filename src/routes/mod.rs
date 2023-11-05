use aide::openapi::OpenApi;
use axum::{
    body::Body,
    routing::get,
    response::IntoResponse,
    http::Response,
    Extension,
    Json,
    Router
};

mod activities;
mod api;
mod nodeinfo;
mod objects;
mod users;
mod well_known;

// ./hatsu --version
async fn root() -> impl IntoResponse {
    let version = env!("CARGO_PKG_VERSION");
    let codename = "01_ballade";

    Response::new(Body::from(format!("Hatsu v{} \"{}\"", version, codename)))
}

async fn openapi_json(Extension(api): Extension<OpenApi>) -> Json<OpenApi> {
    Json(api)
}

pub fn init() -> Router<(), Body> {
    Router::new()
        .merge(activities::init())
        .merge(api::init())
        .merge(nodeinfo::init())
        .merge(objects::init())
        .merge(users::init())
        .merge(well_known::init())
        .route("/", get(root))
        .route("/openapi.json", get(openapi_json))
}
