use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // DbUser
        // https://github.com/LemmyNet/activitypub-federation-rust/blob/61085a643f05dbb70502b3c519fd666214b7e308/examples/live_federation/objects/person.rs#L16-L27C41
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(User::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(User::Name).string().not_null().unique_key())
                    .col(ColumnDef::new(User::PreferredUsername).string().not_null())
                    .col(ColumnDef::new(User::Summary).string())
                    .col(ColumnDef::new(User::Icon).string())
                    .col(ColumnDef::new(User::Image).string())
                    .col(ColumnDef::new(User::Inbox).string().not_null())
                    .col(ColumnDef::new(User::Outbox).string().not_null())
                    .col(ColumnDef::new(User::Followers).string().not_null())
                    .col(ColumnDef::new(User::Following).string().not_null())
                    .col(ColumnDef::new(User::Local).boolean().not_null())
                    .col(ColumnDef::new(User::PublicKey).string().not_null())
                    .col(ColumnDef::new(User::PrivateKey).string())
                    .col(ColumnDef::new(User::FeedJson).string())
                    .col(ColumnDef::new(User::FeedAtom).string())
                    .col(ColumnDef::new(User::FeedRss).string())
                    .col(ColumnDef::new(User::LastRefreshedAt).string().not_null())
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await?;

        Ok(())
    }
}

/// Learn more at <https://docs.rs/sea-query#iden>
#[derive(Iden)]
pub enum User {
    Table,
    Id,
    Name,
    PreferredUsername,
    Summary,
    Icon,
    /// will be remove in v1.0.0
    // #[deprecated(since = "0.2.0-beta.5", note = "replaced by hatsu")]
    Image,
    Inbox,
    Outbox,
    Followers,
    Following,
    Local,
    PublicKey,
    PrivateKey,
    /// Hatsu JSON Feed Extension (`m20240515_000001`)
    ///
    /// <https://hatsu.cli.rs/others/json-feed-extension.html#top-level>
    Hatsu,
    /// User Feed
    Feed,
    /// will be remove in v1.0.0
    // #[deprecated(since = "0.2.0-beta.5", note = "replaced by feed")]
    FeedJson,
    /// will be remove in v1.0.0
    // #[deprecated(since = "0.2.0-beta.5", note = "replaced by feed")]
    FeedAtom,
    /// will be remove in v1.0.0
    // #[deprecated(since = "0.2.0-beta.5", note = "replaced by feed")]
    FeedRss,
    LastRefreshedAt,
}
