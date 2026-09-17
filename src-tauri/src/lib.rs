mod accounts_file;
mod import;
mod secret;
mod storage;

use std::{
    env,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};
use tauri::{AppHandle, Manager};

pub(crate) fn now_millis() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
}

fn resolve_database_path(app: &AppHandle) -> PathBuf {
    if let Ok(override_path) = env::var("WORKSPACE_DB") {
        if !override_path.trim().is_empty() {
            return PathBuf::from(override_path);
        }
    }

    if cfg!(debug_assertions) {
        return env::current_dir()
            .unwrap_or_else(|_| PathBuf::from("."))
            .join("workspace.db");
    }

    app.path()
        .app_data_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .join("workspace.db")
}

#[tauri::command]
fn load_accounts(app: AppHandle) -> accounts_file::AccountLoadResult {
    accounts_file::load_account_file(&accounts_file::resolve_accounts_path(&app))
}

#[tauri::command]
fn import_accounts_file(app: AppHandle) -> Result<import::ImportReport, String> {
    let path = accounts_file::resolve_accounts_path(&app);
    let content = std::fs::read_to_string(&path)
        .map_err(|error| format!("accounts.txt를 읽지 못했습니다: {error}"))?;
    let parsed = accounts_file::parse_accounts(&content);

    let database = storage::Database::open(&resolve_database_path(&app))
        .map_err(|error| format!("저장소를 열지 못했습니다: {error}"))?;
    let secrets = secret::KeychainStore::new();

    import::import_accounts(&database, &secrets, &parsed).map_err(|error| error.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            load_accounts,
            import_accounts_file
        ])
        .run(tauri::generate_context!())
        .expect("Tauri 프로그램을 실행하지 못했습니다.");
}
