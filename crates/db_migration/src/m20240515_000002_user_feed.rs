use sea_orm_migration::prelude::*;

use crate::m20240131_000001_user::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(User::Table)
                    .add_column(ColumnDef::new(User::Feed).json())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(User::Table)
                    .drop_column(User::FeedAtom)
                    .drop_column(User::FeedJson)
                    .drop_column(User::FeedRss)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(User::Table)
                    .drop_column(User::Feed)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(User::Table)
                    .add_column(ColumnDef::new(User::FeedJson).string())
                    .add_column(ColumnDef::new(User::FeedAtom).string())
                    .add_column(ColumnDef::new(User::FeedRss).string())
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}
