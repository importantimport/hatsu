use sea_orm::DatabaseConnection;

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
