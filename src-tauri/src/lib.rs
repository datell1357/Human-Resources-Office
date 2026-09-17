mod accounts_file;
mod import;
mod secret;
mod storage;

use std::{
    env,
    path::PathBuf,
    sync::Mutex,
    time::{SystemTime, UNIX_EPOCH},
};
use storage::{posts, Database};
use tauri::{AppHandle, Manager, State};

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

pub struct AppState {
    database: Mutex<Database>,
}

impl AppState {
    fn read<T>(
        &self,
        action: impl FnOnce(&rusqlite::Connection) -> rusqlite::Result<T>,
    ) -> Result<T, String> {
        let guard = self
            .database
            .lock()
            .map_err(|_| "저장소 잠금을 얻지 못했습니다.".to_string())?;
        action(guard.connection()).map_err(|error| format!("저장소 오류: {error}"))
    }
}

#[tauri::command]
fn load_accounts(app: AppHandle) -> accounts_file::AccountLoadResult {
    accounts_file::load_account_file(&accounts_file::resolve_accounts_path(&app))
}

#[tauri::command]
fn import_accounts_file(
    app: AppHandle,
    state: State<'_, AppState>,
) -> Result<import::ImportReport, String> {
    let path = accounts_file::resolve_accounts_path(&app);
    let content = std::fs::read_to_string(&path)
        .map_err(|error| format!("accounts.txt를 읽지 못했습니다: {error}"))?;
    let parsed = accounts_file::parse_accounts(&content);

    let database = state
        .database
        .lock()
        .map_err(|_| "저장소 잠금을 얻지 못했습니다.".to_string())?;
    let secrets = secret::KeychainStore::new();

    import::import_accounts(&database, &secrets, &parsed).map_err(|error| error.to_string())
}

#[tauri::command]
fn list_posts(state: State<'_, AppState>) -> Result<Vec<posts::Post>, String> {
    state.read(posts::list)
}

#[tauri::command]
fn save_post(state: State<'_, AppState>, input: posts::PostInput) -> Result<posts::Post, String> {
    if input.title.trim().is_empty() {
        return Err("제목을 입력해 주세요.".to_string());
    }
    if input.body.trim().is_empty() {
        return Err("본문을 입력해 주세요.".to_string());
    }
    state.read(|connection| posts::save(connection, &input))
}

#[tauri::command]
fn delete_post(state: State<'_, AppState>, id: i64) -> Result<bool, String> {
    state.read(|connection| posts::delete(connection, id))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let path = resolve_database_path(app.handle());
            let database = Database::open(&path)?;
            app.manage(AppState {
                database: Mutex::new(database),
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            load_accounts,
            import_accounts_file,
            list_posts,
            save_post,
            delete_post
        ])
        .run(tauri::generate_context!())
        .expect("Tauri 프로그램을 실행하지 못했습니다.");
}
