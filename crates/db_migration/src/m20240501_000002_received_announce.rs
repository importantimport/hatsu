use sea_orm_migration::prelude::*;

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
                    .col(
                        ColumnDef::new(ReceivedAnnounce::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ReceivedAnnounce::Actor).string().not_null())
                    .col(ColumnDef::new(ReceivedAnnounce::Object).string().not_null())
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
