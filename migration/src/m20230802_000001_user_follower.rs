use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserFollower::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(UserFollower::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(UserFollower::UserId).string().not_null())
                    .col(ColumnDef::new(UserFollower::FollowerId).string().not_null())
                    .to_owned()
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserFollower::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(Iden)]
enum UserFollower {
    Table,
    // UUID
    Id,
    // 被关注者 ID
    UserId,
    // 关注者 ID
    FollowerId,
}