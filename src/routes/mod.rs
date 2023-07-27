use axum::{
    body::Body,
    routing::get,
    Router,
    response::IntoResponse,
    http::Response
};

mod nodeinfo;
mod objects;
mod users;
mod well_known;

// Hatsu & Version
async fn root() -> impl IntoResponse {
    let version = option_env!("CARGO_PKG_VERSION").unwrap();
    let message = format!("Hatsu\nVersion {}", version);

    Response::new(Body::from(message))
}

pub fn init() -> Router<(), Body> {
    let routes = Router::new()
        .merge(nodeinfo::init())
        .merge(objects::init())
        .merge(users::init())
        .merge(well_known::init())
        .route("/", get(root));

    routes
}