use sea_orm_migration::{
    prelude::*,
    schema::{boolean, string, string_null},
};

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
                    .col(string(User::Id).primary_key())
                    .col(string(User::Name))
                    .col(string(User::PreferredUsername))
                    .col(string_null(User::Summary))
                    .col(string_null(User::Icon))
                    .col(string_null(User::Image))
                    .col(string(User::Inbox))
                    .col(string(User::Outbox))
                    .col(string(User::Followers))
                    .col(string(User::Following))
                    .col(boolean(User::Local))
                    .col(string(User::PublicKey))
                    .col(string_null(User::PrivateKey))
                    .col(string_null(User::FeedJson))
                    .col(string_null(User::FeedAtom))
                    .col(string_null(User::FeedRss))
                    .col(string(User::LastRefreshedAt))
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
    /// language (`m20241028_000001`)
    Language,
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
