use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Activity::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Activity::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Activity::Activity).json().not_null())
                    .col(ColumnDef::new(Activity::Actor).string().not_null())
                    .col(ColumnDef::new(Activity::Kind).string().not_null())
                    .col(ColumnDef::new(Activity::Published).string())
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Activity::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(Iden)]
enum Activity {
    Table,
    // Activity UUID
    Id,
    // Activity JSON
    /// <https://www.w3.org/TR/activitystreams-vocabulary/#activity-types>
    #[allow(clippy::enum_variant_names)]
    Activity,
    // Activity Actor
    Actor,
    // Activity Type
    Kind,
    // Activity Publish Date (optional)
    Published,
}
