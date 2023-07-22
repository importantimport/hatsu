use std::{env, net::SocketAddr};

use activitypub_federation::config::{FederationConfig, FederationMiddleware};
use axum::Router;
use dotenvy::dotenv;
use migration::{Migrator, MigratorTrait};
use sea_orm::*;

mod entities;
use entities::{prelude::*, *};

mod error;
use error::Error;

mod objects;

mod routes;

#[derive(Clone, Debug)]
pub struct AppData {
    conn: DatabaseConnection,
}

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

    Migrator::up(&conn, None)
        .await
        .expect("Migration failed");

    tracing::info!("creating test account");
    let test_account = user::ActiveModel {
        ..user::Model::new(
            env::var("HATSU_TEST_ACCOUNT").expect("DATABASE_URL must be set").as_str()
        ).unwrap().into_active_model()
    };
    let _insert_account = User::insert(test_account)
        .on_conflict(
            sea_query::OnConflict::column(user::Column::Id)
                .update_column(user::Column::Id)
                .to_owned()
        )
        .exec(&conn)
        .await?;

    tracing::info!("setup configuration");
    let config = FederationConfig::builder()
        .domain(env::var("HATSU_DOMAIN").expect("HATSU_DOMAIN must be set"))
        .app_data(AppData {conn})
        .build()
        .await?;

    // build our application with a route
    tracing::info!("creating app");
    let app = Router::new()
        .merge(routes::init())
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

    // ENV TODO: HATSU_LISTEN `127.0.0.1:3939`
    // axum 0.7
    // run our app with hyper
    // let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
    //     .await
    //     .unwrap();
    // tracing::debug!("listening on {}", listener.local_addr().unwrap());
    // axum::serve(listener, app).await.unwrap();

    Ok(())
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
