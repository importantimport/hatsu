use sea_orm_migration::{prelude::*, schema::*};

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
                    .col(string(Activity::Id).primary_key())
                    .col(json(Activity::Activity))
                    .col(string(Activity::Actor))
                    .col(string(Activity::Kind))
                    .col(string_null(Activity::Published))
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
    // Activity URL
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
