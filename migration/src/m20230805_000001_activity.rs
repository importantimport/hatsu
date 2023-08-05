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
                    .col(ColumnDef::new(Activity::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(Activity::Kind).string().not_null())
                    .col(ColumnDef::new(Activity::Actor).string().not_null())
                    .col(ColumnDef::new(Activity::ObjectId).string().not_null())
                    .col(ColumnDef::new(Activity::To).string())
                    .col(ColumnDef::new(Activity::Cc).string())
                    .to_owned()
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

/// https://www.w3.org/TR/activitystreams-vocabulary/#activity-types
#[derive(Iden)]
enum Activity {
    Table,
    // Activity URL
    Id,
    // Activity Type
    Kind,
    // Activity Actor
    Actor,
    // Activity Object Id (URL)
    ObjectId,
    // Activity To (Optional) (JSON Array)
    To,
    // Activity Cc (Optional) (JSON Array)
    Cc,
}