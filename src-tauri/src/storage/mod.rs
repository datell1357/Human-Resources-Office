pub mod posts;

use rusqlite::{Connection, Result as SqlResult};
use std::path::Path;

const SCHEMA: &str = include_str!("schema.sql");
const SCHEMA_VERSION: i64 = 1;

pub struct Database {
    connection: Connection,
}

impl Database {
    pub fn open(path: &Path) -> SqlResult<Self> {
        if let Some(directory) = path.parent() {
            let _ = std::fs::create_dir_all(directory);
        }
        Self::prepare(Connection::open(path)?)
    }

    #[cfg_attr(not(test), allow(dead_code))]
    pub fn open_in_memory() -> SqlResult<Self> {
        Self::prepare(Connection::open_in_memory()?)
    }

    fn prepare(connection: Connection) -> SqlResult<Self> {
        connection.pragma_update(None, "journal_mode", "WAL")?;
        connection.pragma_update(None, "foreign_keys", true)?;
        let database = Self { connection };
        database.migrate()?;
        Ok(database)
    }

    fn migrate(&self) -> SqlResult<()> {
        let current: i64 = self
            .connection
            .query_row("PRAGMA user_version", [], |row| row.get(0))?;
        if current >= SCHEMA_VERSION {
            return Ok(());
        }
        self.connection.execute_batch(SCHEMA)?;
        self.connection
            .pragma_update(None, "user_version", SCHEMA_VERSION)?;
        Ok(())
    }

    pub fn connection(&self) -> &Connection {
        &self.connection
    }

    #[cfg_attr(not(test), allow(dead_code))]
    pub fn schema_version(&self) -> SqlResult<i64> {
        self.connection
            .query_row("PRAGMA user_version", [], |row| row.get(0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn migration_creates_every_table_once() {
        let database = Database::open_in_memory().unwrap();
        assert_eq!(database.schema_version().unwrap(), SCHEMA_VERSION);

        let tables: Vec<String> = database
            .connection()
            .prepare("SELECT name FROM sqlite_master WHERE type = 'table' ORDER BY name")
            .unwrap()
            .query_map([], |row| row.get(0))
            .unwrap()
            .map(Result::unwrap)
            .collect();

        for expected in ["accounts", "posts", "run_items", "runs", "sites"] {
            assert!(tables.contains(&expected.to_string()), "{expected} 누락");
        }

        database.migrate().unwrap();
        assert_eq!(database.schema_version().unwrap(), SCHEMA_VERSION);
    }

    #[test]
    fn run_items_reject_duplicate_targets_in_one_run() {
        let database = Database::open_in_memory().unwrap();
        let connection = database.connection();
        let now = 1_700_000_000_000i64;

        connection
            .execute(
                "INSERT INTO sites (name, url, created_at) VALUES ('예시', 'https://example.com', ?1)",
                [now],
            )
            .unwrap();
        connection
            .execute(
                "INSERT INTO accounts (site_id, username, secret_ref, created_at) VALUES (1, 'worker', 'account:1:worker', ?1)",
                [now],
            )
            .unwrap();
        connection
            .execute(
                "INSERT INTO posts (title, body, created_at, updated_at) VALUES ('제목', '본문', ?1, ?1)",
                [now],
            )
            .unwrap();
        connection
            .execute(
                "INSERT INTO runs (post_id, started_at) VALUES (1, ?1)",
                [now],
            )
            .unwrap();

        let insert =
            "INSERT INTO run_items (run_id, site_id, account_id, seq) VALUES (1, 1, 1, ?1)";
        connection.execute(insert, [1]).unwrap();
        assert!(connection.execute(insert, [2]).is_err());
    }

    #[test]
    fn reopening_a_file_database_keeps_rows_and_skips_remigration() {
        let path = std::env::temp_dir().join(format!(
            "hro-storage-test-{}-{}.db",
            std::process::id(),
            crate::now_millis()
        ));
        let _ = std::fs::remove_file(&path);

        {
            let database = Database::open(&path).unwrap();
            database
                .connection()
                .execute(
                    "INSERT INTO sites (name, url, created_at) VALUES ('예시', 'https://example.com', 1)",
                    [],
                )
                .unwrap();
        }

        let reopened = Database::open(&path).unwrap();
        let count: i64 = reopened
            .connection()
            .query_row("SELECT COUNT(*) FROM sites", [], |row| row.get(0))
            .unwrap();

        assert_eq!(count, 1);
        assert_eq!(reopened.schema_version().unwrap(), SCHEMA_VERSION);

        drop(reopened);
        for suffix in ["", "-wal", "-shm"] {
            let _ = std::fs::remove_file(format!("{}{suffix}", path.display()));
        }
    }
}
