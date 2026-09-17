use crate::accounts_file::{AccountStatus, ParsedAccount};
use crate::now_millis;
use crate::secret::{SecretError, SecretStore};
use crate::storage::Database;
use rusqlite::{params, OptionalExtension};
use serde::Serialize;
use url::Url;

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportReport {
    pub sites_created: usize,
    pub accounts_created: usize,
    pub accounts_updated: usize,
    pub skipped: Vec<SkippedRow>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SkippedRow {
    pub line_number: usize,
    pub reason: String,
}

#[derive(Debug)]
pub enum ImportError {
    Database(rusqlite::Error),
    Secret(SecretError),
}

impl std::fmt::Display for ImportError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Database(error) => write!(formatter, "저장소 오류: {error}"),
            Self::Secret(error) => write!(formatter, "{error}"),
        }
    }
}

impl From<rusqlite::Error> for ImportError {
    fn from(error: rusqlite::Error) -> Self {
        Self::Database(error)
    }
}

impl From<SecretError> for ImportError {
    fn from(error: SecretError) -> Self {
        Self::Secret(error)
    }
}

fn canonical_url(value: &str) -> String {
    Url::parse(value)
        .map(|url| url.to_string())
        .unwrap_or_else(|_| value.to_string())
}

fn site_name(value: &str) -> String {
    Url::parse(value)
        .ok()
        .and_then(|url| url.host_str().map(str::to_string))
        .unwrap_or_else(|| value.to_string())
}

/// accounts.txt 판독 결과를 저장소로 옮긴다.
/// 비밀번호는 데이터베이스에 넣지 않고 참조 키만 남긴다.
pub fn import_accounts(
    database: &Database,
    secrets: &dyn SecretStore,
    accounts: &[ParsedAccount],
) -> Result<ImportReport, ImportError> {
    let mut report = ImportReport::default();
    let now = now_millis() as i64;
    let connection = database.connection();

    for account in accounts {
        if account.status == AccountStatus::Error {
            report.skipped.push(SkippedRow {
                line_number: account.line_number,
                reason: account.validation_message.clone(),
            });
            continue;
        }

        let url = canonical_url(&account.reference_url);
        let site_id: i64 = match connection
            .query_row("SELECT id FROM sites WHERE url = ?1", [&url], |row| {
                row.get(0)
            })
            .optional()?
        {
            Some(id) => id,
            None => {
                connection.execute(
                    "INSERT INTO sites (name, url, created_at) VALUES (?1, ?2, ?3)",
                    params![site_name(&url), &url, now],
                )?;
                report.sites_created += 1;
                connection.last_insert_rowid()
            }
        };

        let secret_reference = format!("account:{site_id}:{}", account.username);
        secrets.store(&secret_reference, &account.password)?;

        let status = if account.status == AccountStatus::Active {
            "active"
        } else {
            "excluded"
        };
        let existing: Option<i64> = connection
            .query_row(
                "SELECT id FROM accounts WHERE site_id = ?1 AND username = ?2",
                params![site_id, &account.username],
                |row| row.get(0),
            )
            .optional()?;

        match existing {
            Some(id) => {
                connection.execute(
                    "UPDATE accounts SET secret_ref = ?1, status = ?2, note = ?3, source_line = ?4 WHERE id = ?5",
                    params![
                        &secret_reference,
                        status,
                        &account.note,
                        account.line_number as i64,
                        id
                    ],
                )?;
                report.accounts_updated += 1;
            }
            None => {
                connection.execute(
                    "INSERT INTO accounts (site_id, username, secret_ref, status, note, source_line, created_at) \
                     VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                    params![
                        site_id,
                        &account.username,
                        &secret_reference,
                        status,
                        &account.note,
                        account.line_number as i64,
                        now
                    ],
                )?;
                report.accounts_created += 1;
            }
        }
    }

    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::accounts_file::parse_accounts;
    use crate::secret::MemorySecretStore;
    use rusqlite::Connection;

    fn dump_every_stored_value(connection: &Connection) -> String {
        let mut dump = String::new();
        for table in ["sites", "accounts", "posts", "runs", "run_items"] {
            let mut statement = connection
                .prepare(&format!("SELECT * FROM {table}"))
                .unwrap();
            let column_count = statement.column_count();
            let mut rows = statement.query([]).unwrap();
            while let Some(row) = rows.next().unwrap() {
                for index in 0..column_count {
                    let value: rusqlite::types::Value = row.get(index).unwrap();
                    dump.push_str(&format!("{value:?} "));
                }
            }
        }
        dump
    }

    #[test]
    fn creates_one_site_per_address_and_keeps_every_account() {
        let database = Database::open_in_memory().unwrap();
        let secrets = MemorySecretStore::default();
        let accounts = parse_accounts(
            "true | https://example.com | worker01 | secret-one | 첫 계정\ntrue | https://example.com | worker02 | secret-two | 같은 사이트\nfalse | https://other.example | worker03 | secret-three | 제외됨",
        );

        let report = import_accounts(&database, &secrets, &accounts).unwrap();

        assert_eq!(report.sites_created, 2);
        assert_eq!(report.accounts_created, 3);
        assert!(report.skipped.is_empty());

        let excluded: String = database
            .connection()
            .query_row(
                "SELECT status FROM accounts WHERE username = 'worker03'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(excluded, "excluded");
    }

    #[test]
    fn never_writes_a_password_into_the_database() {
        let database = Database::open_in_memory().unwrap();
        let secrets = MemorySecretStore::default();
        let accounts =
            parse_accounts("true | https://example.com | private-user | never-store-this | 메모");

        import_accounts(&database, &secrets, &accounts).unwrap();

        let dump = dump_every_stored_value(database.connection());
        assert!(!dump.contains("never-store-this"));
        assert!(dump.contains("account:1:private-user"));
        assert_eq!(
            secrets.retrieve("account:1:private-user").unwrap(),
            "never-store-this"
        );
    }

    #[test]
    fn importing_twice_updates_instead_of_duplicating() {
        let database = Database::open_in_memory().unwrap();
        let secrets = MemorySecretStore::default();
        let accounts = parse_accounts("true | https://example.com | worker01 | secret-one | 처음");

        import_accounts(&database, &secrets, &accounts).unwrap();
        let second = import_accounts(&database, &secrets, &accounts).unwrap();

        assert_eq!(second.sites_created, 0);
        assert_eq!(second.accounts_created, 0);
        assert_eq!(second.accounts_updated, 1);

        let site_count: i64 = database
            .connection()
            .query_row("SELECT COUNT(*) FROM sites", [], |row| row.get(0))
            .unwrap();
        let account_count: i64 = database
            .connection()
            .query_row("SELECT COUNT(*) FROM accounts", [], |row| row.get(0))
            .unwrap();
        assert_eq!(site_count, 1);
        assert_eq!(account_count, 1);
    }

    #[test]
    fn reports_error_rows_instead_of_importing_them() {
        let database = Database::open_in_memory().unwrap();
        let secrets = MemorySecretStore::default();
        let accounts = parse_accounts(
            "true | invalid-url | worker01 | secret-one\ntrue | https://example.com | worker02 | secret-two",
        );

        let report = import_accounts(&database, &secrets, &accounts).unwrap();

        assert_eq!(report.accounts_created, 1);
        assert_eq!(report.skipped.len(), 1);
        assert_eq!(report.skipped[0].line_number, 1);
        assert!(report.skipped[0].reason.contains("URL"));
    }
}
