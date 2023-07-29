use std::{env, net::ToSocketAddrs};

use activitypub_federation::config::{FederationConfig, FederationMiddleware};
use axum::Router;
use dotenvy::dotenv;
use migration::{Migrator, MigratorTrait};
use sea_orm::*;

mod activities;

mod entities;
use entities::{
    prelude::*,
    user,
    user::Model as DbUser
};

mod error;
use error::AppError;

mod objects;

mod routes;

mod utilities;

#[derive(Clone, Debug)]
pub struct AppData {
    conn: DatabaseConnection,
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    // initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_target(false)
        .init();

    // Load environment variables from .env file.
    tracing::info!("loading environment variables");
    dotenv()?;

    // environments
    let database_url: String = env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://hatsu.sqlite3".to_string());
    let hatsu_domain: String = env::var("HATSU_DOMAIN").expect("HATSU_DOMAIN must be set");
    let hatsu_listen: String = env::var("HATSU_LISTEN").unwrap_or_else(|_| "localhost:3939".to_string());
    let hatsu_test_account: String = env::var("HATSU_TEST_ACCOUNT").expect("HATSU_TEST_ACCOUNT must be set");

    let conn = Database::connect(database_url)
        .await
        .expect("Database connection failed");

    Migrator::up(&conn, None)
        .await
        .expect("Migration failed");

    tracing::info!("creating test account");
    let test_account = DbUser::new(hatsu_test_account.as_str()).await?.into_active_model();
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
        .domain(hatsu_domain)
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
    let addr = hatsu_listen
        .to_socket_addrs()?
        .next()
        .expect("Failed to lookup domain name");
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    // axum 0.7
    // run our app with hyper
    // let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
    //     .await?;
    // tracing::debug!("listening on {}", listener.local_addr()?);
    // axum::serve(listener, app).await?;

    Ok(())
}
