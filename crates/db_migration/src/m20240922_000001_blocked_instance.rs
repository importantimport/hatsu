use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(BlockedInstance::Table)
                    .if_not_exists()
                    .col(uuid(BlockedInstance::Id).primary_key())
                    .col(string(BlockedInstance::Instance))
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(BlockedInstance::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(Iden)]
enum BlockedInstance {
    Table,
    // UUID v7
    Id,
    // Instance URL
    Instance,
}
