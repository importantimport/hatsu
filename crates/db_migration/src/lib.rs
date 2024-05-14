pub use sea_orm_migration::prelude::*;

mod m20240131_000001_user;
mod m20240131_000002_user_feed_item;
mod m20240131_000003_post;
mod m20240131_000004_activity;
mod m20240131_000005_received_follow;
mod m20240501_000001_received_like;
mod m20240501_000002_received_announce;
mod m20240515_000001_user_feed_hatsu_extension;
mod m20240515_000002_user_feed;

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
            Box::new(m20240501_000001_received_like::Migration),
            Box::new(m20240501_000002_received_announce::Migration),
            Box::new(m20240515_000001_user_feed_hatsu_extension::Migration),
            Box::new(m20240515_000002_user_feed::Migration),
        ]
    }
}
