use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(BlockedActor::Table)
                    .if_not_exists()
                    .col(uuid(BlockedActor::Id).primary_key())
                    .col(string(BlockedActor::Actor))
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(BlockedActor::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(Iden)]
enum BlockedActor {
    Table,
    // UUID v7
    Id,
    // Actor URL
    Actor,
}
