pub use sea_orm_migration::prelude::*;

mod m20240131_000001_user;
mod m20240131_000002_user_feed_item;
mod m20240131_000003_post;
mod m20240131_000004_activity;
mod m20240131_000005_received_follow;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240131_000001_user::Migration),
            Box::new(m20240131_000002_user_feed_item::Migration),
            Box::new(m20240131_000003_post::Migration),
            Box::new(m20240131_000004_activity::Migration),
            Box::new(m20240131_000005_received_follow::Migration),
        ]
    }
}
