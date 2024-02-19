#[cfg(target_arch = "x86_64")]
#[global_allocator]
static ALLOC: snmalloc_rs::SnMalloc = snmalloc_rs::SnMalloc;

use std::ops::Deref;

use activitypub_federation::config::FederationConfig;
use hatsu_apub::actors::ApubUser;
use hatsu_db_migration::{Migrator, MigratorTrait};
use hatsu_db_schema::prelude::User;
use hatsu_utils::{AppData, AppEnv, AppError};
use sea_orm::{ActiveModelTrait, Database, EntityTrait, IntoActiveModel};
use tokio::time::Duration;
use tokio_graceful_shutdown::Toplevel;
use tracing_subscriber::prelude::*;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    // initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_error::ErrorLayer::default())
        .init();

    // Load environment variables from .env file.
    tracing::info!("loading environment variables");
    // dotenvy::dotenv()?;
    if dotenvy::dotenv().is_err() {
        tracing::debug!("No .env file found");
    }

    // 环境变量
    // Environments
    let env = AppEnv::init();

    // 连接数据库
    // Connecting to database
    let conn = Database::connect(&env.hatsu_database_url)
        .await
        .expect("Database connection failed");

    // 运行 SeaORM Migration
    Migrator::up(&conn, None).await?;

    tracing::info!("checking primary account");
    // 尝试读取数据库中的主要账户，如果不存在则创建
    // Try to read primary account in the database, if it doesn't exist then create
    let primary_account: ApubUser = match User::find_by_id(
        hatsu_utils::url::generate_user_url(&env.hatsu_domain, &env.hatsu_primary_account)?
            .to_string(),
    )
    .one(&conn)
    .await?
    {
        Some(db_user) => db_user.into(),
        // 根据域名创建一个 user::ActiveModel
        // Create a user::ActiveModel based on the domain
        None => ApubUser::new(&env.hatsu_domain, &env.hatsu_primary_account)
            .await?
            .deref()
            .clone()
            .into_active_model()
            .insert(&conn)
            .await?
            .into(),
    };

    // 创建 AppData
    let data = AppData {
        conn,
        env: env.clone(),
    };

    tracing::info!("setup configuration");
    let federation_config = FederationConfig::builder()
        // 实例域名，这里使用 `HATSU_DOMAIN` 环境变量
        // instance domain, `HATSU_DOMAIN` environment is used here.
        .domain(&env.hatsu_domain)
        // 使用测试账户作为 Signed fetch actor，以和 GoToSocial 或启用安全模式的 Mastodon 实例交互
        // Use a test account as a Signed fetch actor to interact with GoToSocial or a Mastodon instance with secure mode enabled
        .signed_fetch_actor(&primary_account)
        // Fediverse 应用数据，目前只有数据库连接
        // Fediverse application data, currently only database connections
        .app_data(data)
        // TODO:
        // Disable this configuration when Pleroma supports HTTP Signature draft-11
        // 当 Pleroma 支持 HTTP Signature draft-11 时，禁用此配置
        // https://git.pleroma.social/pleroma/pleroma/-/issues/2939
        .http_signature_compat(true)
        .build()
        .await?;

    // 创建服务
    let scheduler = hatsu_scheduler::Scheduler::new(&federation_config);
    let server = hatsu_backend::Server::new(&federation_config);

    let _result = Toplevel::<AppError>::new()
        // .start("Migrator", move |s| migrator.run(s))
        .start("Scheduler", move |s| scheduler.run(s))
        .start("Server", move |s| server.run(s))
        .catch_signals()
        .handle_shutdown_requests(Duration::from_millis(5000))
        .await;

    Ok(())
}
