use std::env;

use sea_orm::DatabaseConnection;

use crate::AppError;

#[derive(Clone, Debug)]
pub struct AppData {
    pub conn: DatabaseConnection,
    pub env: AppEnv,
}

#[derive(Clone, Debug)]
pub struct AppEnv {
    pub hatsu_database_url: String,
    pub hatsu_domain: String,
    pub hatsu_listen_host: String,
    pub hatsu_listen_port: String,
    pub hatsu_primary_account: String,
    pub hatsu_access_token: Option<String>,
    pub hatsu_node_name: Option<String>,
    pub hatsu_node_description: Option<String>,
}

impl AppEnv {
    pub fn init() -> Result<Self, AppError> {
        Ok(Self {
            hatsu_database_url: env::var("HATSU_DATABASE_URL")
                .unwrap_or_else(|_| String::from("sqlite::memory:")),
            hatsu_domain: env::var("HATSU_DOMAIN")?,
            hatsu_listen_host: env::var("HATSU_LISTEN_HOST")
                .unwrap_or_else(|_| String::from("127.0.0.1")),
            hatsu_listen_port: env::var("HATSU_LISTEN_PORT")
                .unwrap_or_else(|_| String::from("3939")),
            hatsu_primary_account: env::var("HATSU_PRIMARY_ACCOUNT")?,
            hatsu_access_token: env::var("HATSU_ACCESS_TOKEN").ok(),
            hatsu_node_name: env::var("HATSU_NODE_NAME").ok(),
            hatsu_node_description: env::var("HATSU_NODE_DESCRIPTION").ok(),
        })
    }

    pub fn info() -> String {
        let version = env!("CARGO_PKG_VERSION");
        let codename = "celluloid";

        format!("Hatsu v{version} \"{codename}\"")
    }
}
