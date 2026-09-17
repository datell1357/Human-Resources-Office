use crate::now_millis;
use serde::Serialize;
use std::{
    collections::HashSet,
    env, fs, io,
    path::{Path, PathBuf},
};
use tauri::{AppHandle, Manager};
use url::Url;

#[derive(Clone, Copy, PartialEq)]
pub enum AccountStatus {
    Active,
    Excluded,
    Error,
}

impl AccountStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Excluded => "excluded",
            Self::Error => "error",
        }
    }
}

#[derive(Clone)]
pub struct ParsedAccount {
    pub line_number: usize,
    pub enabled: Option<bool>,
    pub reference_url: String,
    pub username: String,
    pub password: String,
    pub note: String,
    pub status: AccountStatus,
    pub decision: String,
    pub validation_message: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountRow {
    id: String,
    line_number: usize,
    enabled: Option<bool>,
    reference_url: String,
    account_label: String,
    note: String,
    status: String,
    decision: String,
    validation_message: String,
}

#[derive(Default, Serialize)]
pub struct AccountSummary {
    total: usize,
    active: usize,
    error: usize,
    excluded: usize,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountLoadResult {
    file_name: String,
    file_path: String,
    loaded_at: u128,
    state: String,
    file_message: String,
    summary: AccountSummary,
    rows: Vec<AccountRow>,
}

fn parse_enabled(value: &str) -> Option<bool> {
    match value.trim().to_ascii_lowercase().as_str() {
        "true" | "yes" | "y" | "1" | "active" | "on" => Some(true),
        "false" | "no" | "n" | "0" | "inactive" | "off" => Some(false),
        _ => None,
    }
}

fn validate_url(value: &str) -> Option<&'static str> {
    match Url::parse(value) {
        Ok(url) if !matches!(url.scheme(), "http" | "https") => {
            Some("참조 주소는 http:// 또는 https://로 시작해야 합니다.")
        }
        Ok(url) if url.host_str().is_none() => Some("참조 주소에 호스트 이름이 없습니다."),
        Ok(_) => None,
        Err(_) => Some("올바른 URL 형식이 아닙니다."),
    }
}

fn duplicate_key(reference_url: &str, username: &str) -> String {
    let normalized_url = Url::parse(reference_url)
        .map(|url| url.to_string())
        .unwrap_or_else(|_| reference_url.to_string());
    format!("{normalized_url}|{username}")
}

fn mask_account(username: &str) -> String {
    let characters: Vec<char> = username.chars().collect();
    match characters.len() {
        0 => "—".to_string(),
        1 => "*".to_string(),
        length => {
            let visible_length = usize::min(3, usize::max(1, length - 2));
            format!(
                "{}***",
                characters[..visible_length].iter().collect::<String>()
            )
        }
    }
}

pub fn parse_accounts(content: &str) -> Vec<ParsedAccount> {
    let mut seen_active_accounts = HashSet::new();
    let mut accounts = Vec::new();

    for (index, raw_line) in content.trim_start_matches('\u{feff}').lines().enumerate() {
        let line_number = index + 1;
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let fields: Vec<&str> = line.split('|').map(str::trim).collect();
        let enabled_text = fields.first().copied().unwrap_or_default();
        let reference_url = fields.get(1).copied().unwrap_or_default().to_string();
        let username = fields.get(2).copied().unwrap_or_default().to_string();
        let password = fields.get(3).copied().unwrap_or_default().to_string();
        let note = fields.get(4..).unwrap_or_default().join(" | ");
        let enabled = parse_enabled(enabled_text);

        let mut validation_message = if fields.len() < 4 {
            "필드는 active | reference_url | username | password 순서로 4개 이상 필요합니다."
                .to_string()
        } else if enabled.is_none() {
            "active 값은 true 또는 false 형식이어야 합니다.".to_string()
        } else if let Some(message) = validate_url(&reference_url) {
            message.to_string()
        } else if username.is_empty() {
            "계정 아이디가 비어 있습니다.".to_string()
        } else if password.is_empty() {
            "비밀번호가 비어 있습니다.".to_string()
        } else {
            String::new()
        };

        let (status, decision) = if validation_message.is_empty() && enabled == Some(false) {
            validation_message = "active 값이 false여서 접속 대상에서 제외했습니다.".to_string();
            (AccountStatus::Excluded, "비활성으로 제외")
        } else if validation_message.is_empty() && enabled == Some(true) {
            let key = duplicate_key(&reference_url, &username);
            if seen_active_accounts.insert(key) {
                validation_message = "유효한 활성 계정입니다.".to_string();
                (AccountStatus::Active, "접속 대상")
            } else {
                validation_message = "앞선 활성 행과 참조 주소·계정이 중복됩니다.".to_string();
                (AccountStatus::Error, "중복 오류")
            }
        } else {
            (AccountStatus::Error, "형식 오류")
        };

        accounts.push(ParsedAccount {
            line_number,
            enabled,
            reference_url,
            username,
            password,
            note,
            status,
            decision: decision.to_string(),
            validation_message,
        });
    }

    accounts
}

fn summarize(accounts: &[ParsedAccount]) -> AccountSummary {
    AccountSummary {
        total: accounts.len(),
        active: accounts
            .iter()
            .filter(|account| account.status == AccountStatus::Active)
            .count(),
        error: accounts
            .iter()
            .filter(|account| account.status == AccountStatus::Error)
            .count(),
        excluded: accounts
            .iter()
            .filter(|account| account.status == AccountStatus::Excluded)
            .count(),
    }
}

fn connection_target_count(accounts: &[ParsedAccount]) -> usize {
    accounts
        .iter()
        .filter(|account| {
            account.status == AccountStatus::Active
                && !account.username.is_empty()
                && !account.password.is_empty()
        })
        .count()
}

fn to_rows(accounts: &[ParsedAccount]) -> Vec<AccountRow> {
    accounts
        .iter()
        .map(|account| AccountRow {
            id: format!("line-{}", account.line_number),
            line_number: account.line_number,
            enabled: account.enabled,
            reference_url: account.reference_url.clone(),
            account_label: mask_account(&account.username),
            note: account.note.clone(),
            status: account.status.as_str().to_string(),
            decision: account.decision.clone(),
            validation_message: account.validation_message.clone(),
        })
        .collect()
}

pub fn load_account_file(path: &Path) -> AccountLoadResult {
    let file_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("accounts.txt")
        .to_string();
    let file_path = path.to_string_lossy().to_string();

    match fs::read_to_string(path) {
        Ok(content) => {
            let accounts = parse_accounts(&content);
            let active_count = connection_target_count(&accounts);
            AccountLoadResult {
                file_name,
                file_path,
                loaded_at: now_millis(),
                state: "ready".to_string(),
                file_message: format!(
                    "{}개 계정 행을 읽고 {active_count}개 접속 대상을 선택했습니다.",
                    accounts.len()
                ),
                summary: summarize(&accounts),
                rows: to_rows(&accounts),
            }
        }
        Err(error) => AccountLoadResult {
            file_name,
            file_path,
            loaded_at: now_millis(),
            state: if error.kind() == io::ErrorKind::NotFound {
                "missing"
            } else {
                "unreadable"
            }
            .to_string(),
            file_message: if error.kind() == io::ErrorKind::NotFound {
                "accounts.txt를 찾지 못했습니다."
            } else {
                "accounts.txt를 읽는 중 오류가 발생했습니다."
            }
            .to_string(),
            summary: AccountSummary::default(),
            rows: Vec::new(),
        },
    }
}

pub fn resolve_accounts_path(app: &AppHandle) -> PathBuf {
    if let Ok(override_path) = env::var("ACCOUNTS_FILE") {
        if !override_path.trim().is_empty() {
            return PathBuf::from(override_path);
        }
    }

    let working_path = env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .join("accounts.txt");

    if cfg!(debug_assertions) || working_path.exists() {
        return working_path;
    }

    if let Ok(executable) = env::current_exe() {
        if let Some(directory) = executable.parent() {
            let portable_path = directory.join("accounts.txt");
            if portable_path.exists() {
                return portable_path;
            }
        }
    }

    app.path()
        .app_data_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .join("accounts.txt")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn separates_active_excluded_and_error_rows() {
        let accounts = parse_accounts(
            "# comment\ntrue | https://example.com | worker01 | secret-one | 정상\nfalse | https://disabled.example | worker02 | secret-two | 제외\ntrue | invalid-url | worker03 | secret-three | 오류",
        );
        let summary = summarize(&accounts);

        assert_eq!(summary.total, 3);
        assert_eq!(summary.active, 1);
        assert_eq!(summary.error, 1);
        assert_eq!(summary.excluded, 1);
    }

    #[test]
    fn validates_required_fields_and_protocol() {
        let accounts = parse_accounts(
            "maybe | https://example.com | user | password\ntrue | ftp://example.com | user | password\ntrue | https://example.com | | password\ntrue | https://example.com | user |",
        );
        let messages: Vec<&str> = accounts
            .iter()
            .map(|account| account.validation_message.as_str())
            .collect();

        assert_eq!(
            messages,
            vec![
                "active 값은 true 또는 false 형식이어야 합니다.",
                "참조 주소는 http:// 또는 https://로 시작해야 합니다.",
                "계정 아이디가 비어 있습니다.",
                "비밀번호가 비어 있습니다."
            ]
        );
    }

    #[test]
    fn rejects_the_second_active_duplicate() {
        let accounts = parse_accounts(
            "true | https://example.com | worker | first-secret\ntrue | https://example.com | worker | second-secret",
        );

        assert_eq!(accounts[0].status.as_str(), "active");
        assert_eq!(accounts[1].status.as_str(), "error");
        assert!(accounts[1].validation_message.contains("중복"));
    }

    #[test]
    fn renderer_rows_do_not_contain_raw_credentials() {
        let accounts =
            parse_accounts("true | https://example.com | private-user | never-expose-this");
        let serialized = serde_json::to_string(&to_rows(&accounts)).unwrap();

        assert!(serialized.contains("pri***"));
        assert!(!serialized.contains("private-user"));
        assert!(!serialized.contains("never-expose-this"));
    }

    #[test]
    fn example_file_matches_the_documented_summary() {
        let accounts = parse_accounts(include_str!("../../accounts.example.txt"));
        let summary = summarize(&accounts);

        assert_eq!(summary.total, 6);
        assert_eq!(summary.active, 4);
        assert_eq!(summary.error, 1);
        assert_eq!(summary.excluded, 1);
        assert_eq!(
            accounts
                .iter()
                .filter(|account| !account.password.is_empty())
                .count(),
            6
        );
    }
}
