use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserFeedItem::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserFeedItem::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(UserFeedItem::UserId).string().not_null())
                    .col(ColumnDef::new(UserFeedItem::PostId).string())
                    .col(ColumnDef::new(UserFeedItem::Title).string())
                    .col(ColumnDef::new(UserFeedItem::Summary).string())
                    .col(ColumnDef::new(UserFeedItem::Language).string())
                    .col(ColumnDef::new(UserFeedItem::Tags).string())
                    .col(ColumnDef::new(UserFeedItem::DatePublished).string())
                    .col(ColumnDef::new(UserFeedItem::DateModified).string())
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserFeedItem::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(Iden)]
pub enum UserFeedItem {
    Table,
    /// Hatsu JSON Feed Item Extension (`m20240515_000001`)
    ///
    /// <https://hatsu.cli.rs/others/json-feed-extension.html#items>
    Hatsu,
    /// JSON Feed Item `id` or `url`
    Id,
    /// User ID associated with this feed item.
    UserId,
    /// Post ID associated with this feed item.
    PostId,
    /// JSON Feed Item `title`
    Title,
    /// JSON Feed Item `summary`
    Summary,
    /// JSON Feed Item `language`
    Language,
    /// JSON Feed Item `tags`
    Tags,
    /// JSON Feed Item `date_published`
    DatePublished,
    /// JSON Feed Item `date_modified`
    DateModified,
}
