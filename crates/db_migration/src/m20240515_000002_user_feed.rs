use sea_orm_migration::{prelude::*, schema::*};

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
                    .add_column(json_null(User::Feed))
                    .drop_column(User::FeedJson)
                    .drop_column(User::FeedAtom)
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
                    .add_column(string_null(User::FeedJson))
                    .add_column(string_null(User::FeedAtom))
                    .add_column(string_null(User::FeedRss))
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}
