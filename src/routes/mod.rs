use axum::{
    body::Body,
    routing::get,
    Router,
    response::IntoResponse,
    http::Response
};

mod activities;
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

pub fn init() -> Router<(), Body> {
    Router::new()
        .merge(activities::init())
        .merge(nodeinfo::init())
        .merge(objects::init())
        .merge(users::init())
        .merge(well_known::init())
        .route("/", get(root))
}