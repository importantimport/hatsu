use deadpool_sqlite::{Config, Pool, Runtime};

use crate::error::Error;

#[allow(dead_code)]
#[derive(Clone)]
pub struct Database {
    pub pool: Pool,
}

impl Database {
    pub async fn open(path: &str) -> Result<Self, Error> {
        let config = Config::new(path);
        let pool = config.create_pool(Runtime::Tokio1)?;

        let _migration = pool
            .get()
            .await?
            .interact(move |conn| {
                let sql = "create table if not exists users (
                    id text not null primary key,
                    name text not null unique,
                    preferred_username text not null,
                    inbox text not null,
                    outbox text not null,
                    local boolean not null,
                    public_key text not null,
                    private_key text,
                    last_refreshed_at datetime not null
                )";

                conn.execute(sql, ())
            })
            .await
            .map_err(|err| anyhow::anyhow!(format!("{}", err)));

        Ok(Self { pool })
    }
}
