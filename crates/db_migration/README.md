# hatsu_db_migration

## Usage

```bash
# via just
just db_migration [COMMAND] [OPTIONS]
# via sea-orm-cli
sea-orm-cli migrate [COMMAND] [OPTIONS] -d crates/db_migration
# via cargo
cargo run -p hatsu_db_migration -- [COMMAND] [OPTIONS]
# via cargo (in this directory)
cargo run -- [COMMAND] [OPTIONS]
```

### Options

- Generate a new migration file
    ```sh
    cargo run -- migrate generate MIGRATION_NAME
    ```
- Apply all pending migrations
    ```sh
    cargo run
    ```
    ```sh
    cargo run -- up
    ```
- Apply first 10 pending migrations
    ```sh
    cargo run -- up -n 10
    ```
- Rollback last applied migrations
    ```sh
    cargo run -- down
    ```
- Rollback last 10 applied migrations
    ```sh
    cargo run -- down -n 10
    ```
- Drop all tables from the database, then reapply all migrations
    ```sh
    cargo run -- fresh
    ```
- Rollback all applied migrations, then reapply all migrations
    ```sh
    cargo run -- refresh
    ```
- Rollback all applied migrations
    ```sh
    cargo run -- reset
    ```
- Check the status of all migrations
    ```sh
    cargo run -- status
    ```
