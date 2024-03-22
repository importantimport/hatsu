use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ReceivedFollow::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ReceivedFollow::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ReceivedFollow::Actor).string().not_null())
                    .col(ColumnDef::new(ReceivedFollow::To).text())
                    .col(ColumnDef::new(ReceivedFollow::Object).string().not_null())
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ReceivedFollow::Table).to_owned())
            .await?;

        Ok(())
    }
}

/// <https://www.w3.org/ns/activitystreams#Follow>
#[derive(Iden)]
enum ReceivedFollow {
    Table,
    // Follow Url
    Id,
    // 关注者 ID
    Actor,
    // 可选，兼容性
    To,
    // 被关注者 Id
    Object,
}
