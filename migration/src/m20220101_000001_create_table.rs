use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // DbPost
        // https://github.com/LemmyNet/activitypub-federation-rust/blob/61085a643f05dbb70502b3c519fd666214b7e308/examples/live_federation/objects/post.rs#L20C4-L25
        manager
            .create_table(
                Table::create()
                    .table(Post::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Post::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    // .col(ColumnDef::new(Post::Title).string().not_null())
                    .col(ColumnDef::new(Post::Creator).string().not_null())
                    .col(ColumnDef::new(Post::Text).string().not_null())
                    .col(ColumnDef::new(Post::Local).boolean().not_null())
                    .to_owned(),
            )
            .await?;

        // DbUser
        // https://github.com/LemmyNet/activitypub-federation-rust/blob/61085a643f05dbb70502b3c519fd666214b7e308/examples/live_federation/objects/person.rs#L16-L27C41
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(User::Name).string().not_null().unique_key())
                    .col(ColumnDef::new(User::PreferredUsername).string().not_null())
                    .col(ColumnDef::new(User::Inbox).string().not_null())
                    .col(ColumnDef::new(User::Outbox).string().not_null())
                    .col(ColumnDef::new(User::Local).boolean().not_null())
                    .col(ColumnDef::new(User::PublicKey).string().not_null())
                    .col(ColumnDef::new(User::PrivateKey).string())
                    .col(ColumnDef::new(User::LastRefreshedAt).date_time().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(UserFollower::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(UserFollower::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(UserFollower::UserID).string().not_null())
                    .col(ColumnDef::new(UserFollower::FollowerID).string().not_null())
                    .to_owned()
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Post::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(UserFollower::Table).to_owned())
            .await?;

        Ok(())
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Post {
    Table,
    Id,
    Creator,
    Text,
    Local,
}

#[derive(Iden)]
enum User {
    Table,
    Id,
    Name,
    PreferredUsername,
    Inbox,
    Outbox,
    Local,
    PublicKey,
    PrivateKey,
    LastRefreshedAt,
}

#[derive(Iden)]
enum UserFollower {
    Table,
    // UUID
    Id,
    // 被关注者 ID
    UserID,
    // 关注者 ID
    FollowerID,
}
