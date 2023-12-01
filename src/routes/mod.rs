use axum::{
    body::Body,
    routing::get,
    response::IntoResponse,
    http::Response,
    Router
};

mod activities;
mod api;
mod nodeinfo;
mod objects;
mod users;
mod well_known;

mod openapi;

// ./hatsu --version
async fn root() -> impl IntoResponse {
    let version = env!("CARGO_PKG_VERSION");
    let codename = "01_ballade";

    Response::new(Body::from(format!("Hatsu v{} \"{}\"", version, codename)))
}

pub fn handler() -> Router<(), Body> {
    Router::new()
        .merge(activities::handler())
        .merge(api::handler())
        .merge(nodeinfo::handler())
        .merge(objects::handler())
        .merge(users::handler())
        .merge(well_known::handler())
        .merge(openapi::handler())
        .route("/", get(root))
}
