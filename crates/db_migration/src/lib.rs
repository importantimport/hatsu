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
mod m20240926_000001_blocked_url;
mod m20241028_000001_user_language;

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
            Box::new(m20240926_000001_blocked_url::Migration),
            Box::new(m20241028_000001_user_language::Migration),
        ]
    }
}

#[cfg(test)]
mod tests {
    use sea_orm_migration::prelude::sea_orm::{ConnectionTrait, Database, DbBackend, Statement};

    use super::*;

    #[tokio::test]
    async fn sqlite_migrations_are_idempotent_and_preserve_existing_data() {
        let db = Database::connect("sqlite::memory:").await.unwrap();

        Migrator::up(&db, None).await.unwrap();

        db.execute_unprepared(
            r#"
            INSERT INTO "user" (
                "id", "name", "preferred_username", "summary", "icon",
                "inbox", "outbox", "followers", "following", "local",
                "public_key", "private_key", "last_refreshed_at", "hatsu",
                "feed", "language"
            ) VALUES (
                'https://example.test/users/alice', 'Alice', 'alice', NULL, NULL,
                'https://example.test/users/alice/inbox',
                'https://example.test/users/alice/outbox',
                'https://example.test/users/alice/followers',
                'https://example.test/users/alice/following', 1,
                'test-public-key', NULL, '2026-08-24T00:00:00Z', NULL, NULL, NULL
            )
            "#,
        )
        .await
        .unwrap();

        let migration_count = Statement::from_string(
            DbBackend::Sqlite,
            "SELECT COUNT(*) AS count FROM seaql_migrations".to_owned(),
        );
        let migration_count: i64 = db
            .query_one_raw(migration_count)
            .await
            .unwrap()
            .unwrap()
            .try_get("", "count")
            .unwrap();
        assert_eq!(migration_count as usize, Migrator::migrations().len());

        // A database upgraded in place must not re-run or remove existing data.
        Migrator::up(&db, None).await.unwrap();

        let user = Statement::from_string(
            DbBackend::Sqlite,
            "SELECT id, name FROM \"user\" WHERE id = 'https://example.test/users/alice'"
                .to_owned(),
        );
        let user = db.query_one_raw(user).await.unwrap().unwrap();
        let id: String = user.try_get("", "id").unwrap();
        let name: String = user.try_get("", "name").unwrap();
        assert_eq!(id, "https://example.test/users/alice");
        assert_eq!(name, "Alice");
    }
}
