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
                    .col(ColumnDef::new(Post::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(Post::Object).text().not_null())
                    .col(ColumnDef::new(Post::AttributedTo).string().not_null())
                    .col(ColumnDef::new(Post::InReplyTo).string())
                    .col(ColumnDef::new(Post::InReplyToRoot).string())
                    .col(ColumnDef::new(Post::Published).string().not_null())
                    .col(ColumnDef::new(Post::Updated).string())
                    .col(ColumnDef::new(Post::LastRefreshedAt).string().not_null())
                    .col(ColumnDef::new(Post::Local).boolean().not_null())
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Post::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(Iden)]
enum Post {
    Table,
    // Object ID
    Id,
    // Object JSON
    /// <https://www.w3.org/TR/activitystreams-vocabulary/#object-types>
    Object,
    // 作者
    // Author
    AttributedTo,
    // 回复帖文
    InReplyTo,
    // 顶层回复帖文
    InReplyToRoot,
    // 发布时间
    Published,
    // 更新时间
    Updated,
    // 最后更新时间
    LastRefreshedAt,
    // 是否为本地
    Local,
}
