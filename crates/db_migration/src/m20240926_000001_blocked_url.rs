use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(BlockedUrl::Table)
                    .if_not_exists()
                    .col(string(BlockedUrl::Id).primary_key())
                    .col(boolean(BlockedUrl::IsInstance))
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(BlockedUrl::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(Iden)]
enum BlockedUrl {
    Table,
    // Url
    Id,
    // is instance (if false, then this is actor)
    IsInstance,
}
