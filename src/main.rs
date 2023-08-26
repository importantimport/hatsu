use std::env;

use dotenvy::dotenv;
use migration::{Migrator, MigratorTrait};
use sea_orm::*;
use tokio::time::Duration;
use tokio_graceful_shutdown::Toplevel;

mod entities;
use entities::{
    prelude::*,
    user::Model as DbUser
};

mod error;
use error::AppError;

mod protocol;

mod routes;

mod utilities;

// Subsystem
mod scheduler;
mod web_server;

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

    // 连接数据库
    // Connecting to database
    let conn = Database::connect(database_url)
        .await
        .expect("Database connection failed");

    // 运行 SeaORM Migration
    // Run SeaORM Migration
    // https://www.sea-ql.org/SeaORM/docs/migration/running-migration/#migrating-programmatically
    Migrator::up(&conn, None)
        .await
        .expect("Migration failed");

    tracing::info!("checking test account");
    // 尝试读取数据库中的测试账户，如果不存在则创建
    // Try to read test account in the database, if it doesn't exist then create
    let test_account: DbUser = match User::find_by_id(format!("https://{}/u/{}", hatsu_domain, hatsu_test_account))
        .one(&conn)
        .await? {
            Some(test_account) => test_account,
            None => {
                // 根据域名创建一个 user::ActiveModel
                // Create a user::ActiveModel based on the domain
                let test_account = DbUser::new(hatsu_test_account.as_str(), &conn).await?.into_active_model();
                // 向数据库插入 user::ActiveModel，并返回一个 user::Model (DbUser)
                // Inserts a user::ActiveModel into the database and returns a user::Model (DbUser).
                test_account.insert(&conn).await?
            }
        };

    // 创建 AppData
    let data = AppData { conn };

    let _result = Toplevel::<AppError>::new()
        .start("Web Server", move |subsys| {
            web_server::init(subsys, data, hatsu_domain, hatsu_listen, test_account)
        })
        .start("Scheduler", move |subsys| {
            scheduler::init(subsys)
        })
        .catch_signals()
        .handle_shutdown_requests(Duration::from_millis(5000))
        .await;

    Ok(())
}
