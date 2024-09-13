use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ReceivedAnnounce::Table)
                    .if_not_exists()
                    .col(string(ReceivedAnnounce::Id).primary_key())
                    .col(string(ReceivedAnnounce::Actor))
                    .col(string(ReceivedAnnounce::Object))
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ReceivedAnnounce::Table).to_owned())
            .await?;

        Ok(())
    }
}

/// <https://www.w3.org/ns/activitystreams#Announce>
#[derive(Iden)]
enum ReceivedAnnounce {
    Table,
    // Announce Activity Url
    Id,
    // Attributed To
    Actor,
    // Announced Post Url
    Object,
}
