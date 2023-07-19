use deadpool_sqlite::{Config, Pool, Runtime};

use crate::error::Error;

#[allow(dead_code)]
#[derive(Clone)]
pub struct Database {
    pool: Pool,
}

impl Database {
    pub async fn new(path: &str) -> Result<Self, Error> {
        let config = Config::new(path);
        let pool = config.create_pool(Runtime::Tokio1)?;

        let _migration = pool
            .get()
            .await?
            .interact(move |conn| {
                let sql = "create table if not exists users (
                    id integer primary key,
                    name text not null unique
                )";

                conn.execute(sql, ())
            })
            .await
            .map_err(|err| anyhow::anyhow!(format!("{}", err)));

        Ok(Self { pool })
    }
}
