use std::env;

use activitypub_federation::config::FederationConfig;
use dotenvy::dotenv;
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

mod subsystem;

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

    // 连接数据库
    // Connecting to database
    let conn = Database::connect(database_url)
        .await
        .expect("Database connection failed");

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
                let test_account = DbUser::new(hatsu_test_account.as_str()).await?.into_active_model();
                // 向数据库插入 user::ActiveModel，并返回一个 user::Model (DbUser)
                // Inserts a user::ActiveModel into the database and returns a user::Model (DbUser).
                test_account.insert(&conn).await?
            }
        };

    // 创建 AppData
    let data = AppData { conn };

    tracing::info!("setup configuration");
    let federation_config = FederationConfig::builder()
        // 实例域名，这里使用 `HATSU_DOMAIN` 环境变量
        // instance domain, `HATSU_DOMAIN` environment is used here.
        .domain(&hatsu_domain)
        // 使用测试账户作为 Signed fetch actor，以和 GoToSocial 或启用安全模式的 Mastodon 实例交互
        // Use a test account as a Signed fetch actor to interact with GoToSocial or a Mastodon instance with secure mode enabled
        .signed_fetch_actor(&test_account)
        // Fediverse 应用数据，目前只有数据库连接
        // Fediverse application data, currently only database connections
        .app_data(data.clone())
        // TODO:
        // Disable this configuration when Pleroma supports HTTP Signature draft-11
        // 当 Pleroma 支持 HTTP Signature draft-11 时，禁用此配置
        // https://git.pleroma.social/pleroma/pleroma/-/issues/2939
        .http_signature_compat(true)
        .build()
        .await?;

    // 创建服务
    let migrator = subsystem::Migrator { data: federation_config.to_request_data() };
    let scheduler = subsystem::Scheduler { config: federation_config.clone() };
    let web_server = subsystem::WebServer {
        federation_config,
        hatsu_domain,
        hatsu_listen,
        test_account
    };

    let _result = Toplevel::<AppError>::new()
        .start("Migrator", move |s| migrator.run(s))
        .start("Scheduler", move |s| scheduler.run(s))
        .start("Web Server", move |s| web_server.run(s))
        .catch_signals()
        .handle_shutdown_requests(Duration::from_millis(5000))
        .await;

    Ok(())
}
