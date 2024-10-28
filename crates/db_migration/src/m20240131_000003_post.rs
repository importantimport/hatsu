use sea_orm_migration::{
    prelude::*,
    schema::{boolean, string, string_null, text},
};

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
                    .col(string(Post::Id).primary_key())
                    .col(text(Post::Object))
                    .col(string(Post::AttributedTo))
                    .col(string_null(Post::InReplyTo))
                    .col(string_null(Post::InReplyToRoot))
                    .col(string(Post::Published))
                    .col(string_null(Post::Updated))
                    .col(string(Post::LastRefreshedAt))
                    .col(boolean(Post::Local))
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
