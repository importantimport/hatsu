use sea_orm_migration::{prelude::*, schema::*};

use crate::{m20240131_000001_user::User, m20240131_000002_user_feed_item::UserFeedItem};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(User::Table)
                    .add_column(json_null(User::Hatsu))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(User::Table)
                    .drop_column(User::Image)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(UserFeedItem::Table)
                    .add_column(json_null(UserFeedItem::Hatsu))
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
                    .drop_column(User::Hatsu)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(User::Table)
                    .add_column(string_null(User::Image))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(UserFeedItem::Table)
                    .drop_column(UserFeedItem::Hatsu)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}
