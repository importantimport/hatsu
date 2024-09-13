use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ReceivedLike::Table)
                    .if_not_exists()
                    .col(string(ReceivedLike::Id).primary_key())
                    .col(string(ReceivedLike::Actor))
                    .col(string(ReceivedLike::Object))
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ReceivedLike::Table).to_owned())
            .await?;

        Ok(())
    }
}

/// <https://www.w3.org/ns/activitystreams#Like>
#[derive(Iden)]
enum ReceivedLike {
    Table,
    // Like Activity Url
    Id,
    // Attributed To
    Actor,
    // Liked Post Url
    Object,
}
