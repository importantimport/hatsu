use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("user"))
                    .add_column_if_not_exists(
                        ColumnDef::new(Alias::new("feed")).json().not_null().default("{}")
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(UserFeed::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(UserFeed::UserId).string().not_null())
                    .col(ColumnDef::new(UserFeed::Hatsu).json().not_null())
                    .col(ColumnDef::new(UserFeed::FeedUrl).string().not_null().primary_key())
                    .col(ColumnDef::new(UserFeed::NextUrl).string())
                    .col(ColumnDef::new(UserFeed::Title).string().not_null())
                    .col(ColumnDef::new(UserFeed::Description).string())
                    .col(ColumnDef::new(UserFeed::Icon).string())
                    .col(ColumnDef::new(UserFeed::Language).string())
                    .col(ColumnDef::new(UserFeed::Items).json().not_null())
                    .to_owned()
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new("user"))
                    .drop_column(Alias::new("feed"))
                    .to_owned()
            )
            .await?;

        manager
            .drop_table(Table::drop().table(UserFeed::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(Iden)]
enum UserFeed {
    Table,
    /// User ID associated with this feed.
    UserId,

    /// Hatsu JSON Feed Extension
    /// https://github.com/importantimport/hatsu/issues/1
    Hatsu,

    // Top Level
    // https://www.jsonfeed.org/version/1.1/#top-level-a-name-top-level-a

    /// (primary key) the URL of the feed.
    FeedUrl,
    /// the URL of a feed that provides the next n items, where n is determined by the publisher.
    NextUrl,

    /// 用户名称 (perferredUsername)
    /// the name of the feed.
    Title,
    /// 用户描述
    /// provides more detail, beyond the title, on what the feed is about.
    Description,
    /// 用户头像
    /// the URL of an image for the feed suitable to be used in a timeline, much the way an avatar might be used.
    Icon,
    /// the primary language for the feed in the format specified in RFC 5646.
    /// TODO: 用于语言标记
    Language,

    /// JSON Feed Items
    /// https://www.jsonfeed.org/version/1.1/#items-a-name-items-a
    Items,
}

