use migration::{Migrator as HatsuMigrator, MigratorTrait};
use tokio_graceful_shutdown::SubsystemHandle;

use crate::{
    AppData,
    AppError,
};

pub struct Migrator {
    pub data: AppData,
}

impl Migrator {
    // 运行 SeaORM Migration
    // Run SeaORM Migration
    // https://www.sea-ql.org/SeaORM/docs/migration/running-migration/#migrating-programmatically
    pub async fn run(self, _subsys: SubsystemHandle<AppError>) -> Result<(), AppError> {
        HatsuMigrator::up(&self.data.conn, None).await?;

        Ok(())
    }
}
