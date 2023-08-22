use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Alias::new("post")).to_owned())
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Post::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Post::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(Post::Object).json().not_null())
                    .col(ColumnDef::new(Post::AttributedTo).string().not_null())
                    .col(ColumnDef::new(Post::Published).date_time().not_null())
                    .col(ColumnDef::new(Post::Updated).date_time())
                    .col(ColumnDef::new(Post::LastRefreshedAt).date_time().not_null())
                    .col(ColumnDef::new(Post::Local).boolean().not_null())
                    .to_owned()
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
    /// https://www.w3.org/TR/activitystreams-vocabulary/#object-types
    Object,
    // 作者
    // Author
    AttributedTo,
    // 发布时间
    Published,
    // 更新时间
    Updated,
    // 最后更新时间
    LastRefreshedAt,
    // 是否为本地
    Local,
}