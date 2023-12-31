pub use sea_orm_migration::prelude::*;

mod m20220101_000001_user;
mod m20230808_000001_activity;
mod m20230822_000001_post;
mod m20230831_000001_user_feed_item;
mod m20230902_000001_received_follow;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_user::Migration),
            Box::new(m20230808_000001_activity::Migration),
            Box::new(m20230822_000001_post::Migration),
            Box::new(m20230831_000001_user_feed_item::Migration),
            Box::new(m20230902_000001_received_follow::Migration),
        ]
    }
}
