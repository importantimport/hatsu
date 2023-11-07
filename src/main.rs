use std::env;

use activitypub_federation::config::FederationConfig;
use dotenvy::dotenv;
use migration::{Migrator, MigratorTrait};
use sea_orm::*;
use tokio::time::Duration;
use tokio_graceful_shutdown::Toplevel;
use tracing_error::ErrorLayer;
use tracing_subscriber::prelude::*;

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
    env: AppEnv,
}

#[derive(Clone, Debug)]
pub struct AppEnv {
    database_url: String,
    hatsu_access_token: Option<String>,
    hatsu_domain: String,
    hatsu_listen: String,
    hatsu_test_account: String,
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    // initialize tracing
    let subscriber = tracing_subscriber::Registry::default()
        .with(
            tracing_subscriber::fmt::layer()
                .with_ansi(false)
                .json()
        )
        .with(ErrorLayer::default());
    
    // TODO: tracing_opentelemetry
    tracing::subscriber::set_global_default(subscriber)?;

    // Load environment variables from .env file.
    tracing::info!("loading environment variables");
    dotenv()?;

    // 环境变量
    // Environments
    let env = AppEnv {
        database_url: env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://hatsu.sqlite3".to_string()),
        hatsu_access_token: env::var_os("HATSU_ACCESS_TOKEN").map(|env| env.into_string().unwrap()),
        hatsu_domain: env::var("HATSU_DOMAIN").expect("env HATSU_DOMAIN must be set"),
        hatsu_listen: env::var("HATSU_LISTEN").unwrap_or_else(|_| "localhost:3939".to_string()),
        hatsu_test_account: env::var("HATSU_TEST_ACCOUNT").expect("env HATSU_TEST_ACCOUNT must be set"),
    };

    // 连接数据库
    // Connecting to database
    let conn = Database::connect(&env.database_url)
        .await
        .expect("Database connection failed");

    // 运行 SeaORM Migration
    Migrator::up(&conn, None).await?;

    tracing::info!("checking test account");
    // 尝试读取数据库中的测试账户，如果不存在则创建
    // Try to read test account in the database, if it doesn't exist then create
    let test_account: DbUser = match User::find_by_id(format!("https://{}/u/{}", env.hatsu_domain, env.hatsu_test_account))
        .one(&conn)
        .await? {
            Some(test_account) => test_account,
            None => {
                // 根据域名创建一个 user::ActiveModel
                // Create a user::ActiveModel based on the domain
                let test_account = DbUser::new(env.hatsu_test_account.as_str()).await?.into_active_model();
                // 向数据库插入 user::ActiveModel，并返回一个 user::Model (DbUser)
                // Inserts a user::ActiveModel into the database and returns a user::Model (DbUser).
                test_account.insert(&conn).await?
            }
        };

    // 创建 AppData
    let data = AppData { conn, env: env.clone() };

    tracing::info!("setup configuration");
    let federation_config = FederationConfig::builder()
        // 实例域名，这里使用 `HATSU_DOMAIN` 环境变量
        // instance domain, `HATSU_DOMAIN` environment is used here.
        .domain(&env.hatsu_domain)
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
    // let migrator = subsystem::Migrator { data: federation_config.to_request_data() };
    let scheduler = subsystem::Scheduler { config: federation_config.clone() };
    let server = subsystem::Server {
        federation_config,
        env: env.clone(),
        test_account
    };

    let _result = Toplevel::<AppError>::new()
        // .start("Migrator", move |s| migrator.run(s))
        .start("Scheduler", move |s| scheduler.run(s))
        .start("Web Server", move |s| server.run(s))
        .catch_signals()
        .handle_shutdown_requests(Duration::from_millis(5000))
        .await;

    Ok(())
}
