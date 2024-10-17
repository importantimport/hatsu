use std::{env, ops::Deref, path::Path};

use activitypub_federation::config::FederationConfig;
use hatsu_apub::actors::ApubUser;
use hatsu_db_migration::{Migrator, MigratorTrait};
use hatsu_db_schema::prelude::User;
use hatsu_utils::{AppData, AppEnv, AppError};
use sea_orm::{ActiveModelTrait, Database, EntityTrait, IntoActiveModel};

pub async fn run() -> Result<(), AppError> {
    hatsu_tracing::init()?;

    tracing::info!("{}", AppEnv::info());

    tracing::info!("loading environment variables");
    if dotenvy::dotenv().is_err() {
        let env_file =
            env::var("HATSU_ENV_FILE").unwrap_or_else(|_| String::from("/etc/hatsu/.dev"));
        if dotenvy::from_path(Path::new(&env_file)).is_err() {
            tracing::debug!("no .env file found");
        }
    }

    let env = AppEnv::init()?;

    tracing::info!("connecting database: {}", &env.hatsu_database_url);
    let conn = Database::connect(&env.hatsu_database_url).await?;

    tracing::info!("running database migration");
    Migrator::up(&conn, None).await?;

    tracing::info!("checking primary account: {}", &env.hatsu_primary_account);
    let primary_account: ApubUser = match User::find_by_id(
        hatsu_utils::url::generate_user_url(&env.hatsu_domain, &env.hatsu_primary_account)?
            .to_string(),
    )
    .one(&conn)
    .await?
    {
        Some(db_user) => db_user.into(),
        None => ApubUser::new(&env.hatsu_domain, &env.hatsu_primary_account)
            .await?
            .deref()
            .clone()
            .into_active_model()
            .insert(&conn)
            .await?
            .into(),
    };

    let data = AppData {
        conn,
        env: env.clone(),
    };

    tracing::info!("setup federation config");
    let federation_config = FederationConfig::builder()
        .domain(&env.hatsu_domain)
        .signed_fetch_actor(&primary_account)
        .app_data(data)
        // TODO:
        // Disable this configuration when Pleroma supports HTTP Signature draft-11
        // 当 Pleroma 支持 HTTP Signature draft-11 时，禁用此配置
        // https://git.pleroma.social/pleroma/pleroma/-/issues/2939
        .http_signature_compat(true)
        .build()
        .await?;

    tracing::info!("starting backend");
    hatsu_backend::run(federation_config).await
}
