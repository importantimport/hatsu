use std::{
    env,
    net::SocketAddr
};

use activitypub_federation::config::{FederationConfig, FederationMiddleware};
use axum::{routing::get, Router};
use dotenvy::dotenv;
use sea_orm::Database;

mod error;
use error::Error;

mod objects;

mod routes;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_target(false)
        .init();

    // Load environment variables from .env file.
    tracing::info!("loading environment variables");
    dotenv()?;

    let conn = Database::connect(env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
        .await
        .expect("Database connection failed");

    tracing::info!("setup configuration");
    let config = FederationConfig::builder()
        .domain(env::var("HATSU_DOMAIN").expect("HATSU_DOMAIN must be set"))
        .app_data(conn)
        .build()
        .await?;

    // build our application with a route
    tracing::info!("creating app");
    let app = Router::new()
        .merge(routes::init())
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        // .route("/users", post(create_user))
        .layer(FederationMiddleware::new(config));

    // axum 0.6
    // run our app with hyper
    let addr = SocketAddr::from(([0, 0, 0, 0], 3939));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    // axum 0.7
    // run our app with hyper
    // let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
    //     .await
    //     .unwrap();
    // tracing::debug!("listening on {}", listener.local_addr().unwrap());
    // axum::serve(listener, app).await.unwrap();

    Ok(())
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

// async fn create_user(
//     // this argument tells axum to parse the request body
//     // as JSON into a `CreateUser` type
//     Json(payload): Json<CreateUser>,
// ) -> impl IntoResponse {
//     // insert your application logic here
//     let user = User {
//         id: 1337,
//         username: payload.username,
//     };

//     // this will be converted into a JSON response
//     // with a status code of `201 Created`
//     (StatusCode::CREATED, Json(user))
// }

// // the input to our `create_user` handler
// #[derive(Deserialize)]
// struct CreateUser {
//     username: String,
// }

// // the output to our `create_user` handler
// #[derive(Serialize)]
// struct User {
//     id: u64,
//     username: String,
// }
