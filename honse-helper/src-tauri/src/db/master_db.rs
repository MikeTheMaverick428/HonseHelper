use chrono::Utc;
use rusqlite::{Connection, OpenFlags};
use shared::MasterDbStatus;
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use tauri::{AppHandle, Emitter, State};
use thiserror::Error;

const TARGET_APP_ID: &str = "3224770";
const XOR_KEY: u8 = 0x5A;

fn xdecode(encoded: &[u8], key: u8) -> String {
    encoded.iter().map(|&b| b ^ key).map(char::from).collect()
}

fn master_db_relative_path() -> String {
    xdecode(
        &[
            0x1B, 0x2A, 0x2A, 0x1E, 0x3B, 0x2E, 0x3B, 0x75, 0x16, 0x35, 0x39, 0x3B, 0x36,
            0x16, 0x35, 0x2D, 0x75, 0x19, 0x23, 0x3D, 0x3B, 0x37, 0x3F, 0x29, 0x75, 0x0F, 0x37,
            0x3B, 0x37, 0x2F, 0x29, 0x2F, 0x37, 0x3F, 0x75, 0x37, 0x3B, 0x29, 0x2E, 0x3F, 0x28,
            0x75, 0x37, 0x3B, 0x29, 0x2E, 0x3F, 0x28, 0x74, 0x37, 0x3E, 0x38,
        ],
        XOR_KEY,
    )
}

#[derive(Debug, Error)]
pub enum MasterDbError {
    #[error("not found")]
    NotFound,

    #[error("database validation failed: {0}")]
    ValidationFailed(String),

    #[error("database error: {0}")]
    Sqlite(#[from] rusqlite::Error),

    #[error("state lock poisoned")]
    StateLockPoisoned,
}

#[derive(Debug, Clone)]
pub(crate) struct MasterDbRuntimeState {
    configured_path: Option<PathBuf>,
    discovered_path: Option<PathBuf>,
    source: String,
    message: String,
    last_checked: Option<String>,
    checked_candidates: Vec<PathBuf>,
}

impl MasterDbRuntimeState {
    fn new(configured_path: Option<PathBuf>) -> Self {
        Self {
            configured_path,
            discovered_path: None,
            source: "pending".to_string(),
            message: "Master DB discovery has not run yet".to_string(),
            last_checked: None,
            checked_candidates: Vec::new(),
        }
    }

    fn snapshot(&self) -> MasterDbStatus {
        MasterDbStatus {
            found: self.discovered_path.is_some(),
            source: self.source.clone(),
            path: self
                .discovered_path
                .as_ref()
                .map(|path| path.display().to_string()),
            message: self.message.clone(),
            last_checked: self.last_checked.clone(),
            checked_candidates: self
                .checked_candidates
                .iter()
                .map(|path| path.display().to_string())
                .collect(),
        }
    }
}

pub struct MasterDbState {
    pub inner: Mutex<MasterDbRuntimeState>,
}

impl MasterDbState {
    pub fn new() -> Self {
        let configured_path = crate::app_config::master_db_config()
            .path
            .filter(|p| Path::new(p).is_file())
            .map(PathBuf::from);
        Self {
            inner: Mutex::new(MasterDbRuntimeState::new(configured_path)),
        }
    }
}

impl Default for MasterDbState {
    fn default() -> Self {
        Self::new()
    }
}

pub fn current_master_db_path(state: &State<'_, MasterDbState>) -> Result<Option<PathBuf>, String> {
    let guard = state
        .inner
        .lock()
        .map_err(|_| MasterDbError::StateLockPoisoned.to_string())?;
    Ok(guard
        .discovered_path
        .clone()
        .or_else(|| guard.configured_path.clone()))
}

#[tauri::command]
pub fn get_master_db_status(state: State<'_, MasterDbState>) -> Result<MasterDbStatus, String> {
    let guard = state
        .inner
        .lock()
        .map_err(|_| MasterDbError::StateLockPoisoned.to_string())?;
    Ok(guard.snapshot())
}

#[tauri::command]
pub fn discover_master_db_path(
    app: AppHandle,
    state: State<'_, MasterDbState>,
) -> Result<MasterDbStatus, String> {
    discover_master_db_path_impl(&app, &state)
}

#[tauri::command]
pub fn set_master_db_path(
    app: AppHandle,
    state: State<'_, MasterDbState>,
    path: String,
) -> Result<MasterDbStatus, String> {
    let candidate = PathBuf::from(path);
    validate_master_db_path(&candidate).map_err(|err| err.to_string())?;

    crate::app_config::update_master_db(&candidate.display().to_string(), "manual");

    let mut guard = state
        .inner
        .lock()
        .map_err(|_| MasterDbError::StateLockPoisoned.to_string())?;
    guard.configured_path = Some(candidate.clone());
    guard.discovered_path = Some(candidate.clone());
    guard.source = "manual".to_string();
    guard.message = format!("Using manual master.mdb path: {}", candidate.display());
    guard.last_checked = Some(Utc::now().to_rfc3339());
    guard.checked_candidates = vec![candidate];

    let snapshot = guard.snapshot();
    let _ = app.emit("master-db-status", snapshot.clone());
    Ok(snapshot)
}

pub fn discover_master_db_path_impl(
    app: &AppHandle,
    state: &State<'_, MasterDbState>,
) -> Result<MasterDbStatus, String> {
    let mut guard = state
        .inner
        .lock()
        .map_err(|_| MasterDbError::StateLockPoisoned.to_string())?;
    guard.last_checked = Some(Utc::now().to_rfc3339());
    guard.checked_candidates.clear();
    guard.discovered_path = None;
    guard.source = "searching".to_string();
    guard.message = "Searching for master.mdb".to_string();

    let candidates = collect_candidate_paths(&guard.configured_path);
    let mut found: Option<(PathBuf, String)> = None;

    for candidate in candidates {
        if guard
            .checked_candidates
            .iter()
            .any(|existing| existing == &candidate)
        {
            continue;
        }

        guard.checked_candidates.push(candidate.clone());

        if !candidate.is_file() {
            continue;
        }

        match validate_master_db_path(&candidate) {
            Ok(()) => {
                found = Some((candidate, "auto".to_string()));
                break;
            }
            Err(_) => continue,
        }
    }

    match found {
        Some((path, source)) => {
            crate::app_config::update_master_db(&path.display().to_string(), &source);
            guard.configured_path = Some(path.clone());
            guard.discovered_path = Some(path.clone());
            guard.source = source;
            guard.message = format!("Found master.mdb at {}", path.display());
        }
        None => {
            guard.discovered_path = None;
            guard.source = "missing".to_string();
            guard.message = "Could not find master.mdb automatically".to_string();
        }
    }

    let snapshot = guard.snapshot();
    let _ = app.emit("master-db-status", snapshot.clone());
    Ok(snapshot)
}

fn validate_master_db_path(path: &Path) -> Result<(), MasterDbError> {
    if !path.exists() {
        return Err(MasterDbError::NotFound);
    }

    let conn = Connection::open_with_flags(path, OpenFlags::SQLITE_OPEN_READ_ONLY)?;
    conn.query_row("PRAGMA schema_version;", [], |_| Ok(()))
        .map_err(|err| MasterDbError::ValidationFailed(err.to_string()))?;
    Ok(())
}

fn collect_candidate_paths(configured_path: &Option<PathBuf>) -> Vec<PathBuf> {
    let mut candidates = Vec::new();
    let mut seen = BTreeSet::new();

    let mut push_candidate = |path: PathBuf| {
        if seen.insert(path.clone()) {
            candidates.push(path);
        }
    };

    if let Ok(path) = std::env::var("HONSE_MASTER_DB_PATH") {
        push_candidate(PathBuf::from(path));
    }

    if let Some(path) = configured_path.clone() {
        push_candidate(path);
    }

    if let Some(saved_path) = crate::app_config::master_db_config()
        .path
        .filter(|p| Path::new(p).is_file())
        .map(PathBuf::from)
    {
        push_candidate(saved_path);
    }

    #[cfg(target_os = "windows")]
    if let Some(home) = dirs::home_dir() {
        push_candidate(
            home.join("AppData")
                .join("LocalLow")
                .join(xdecode(&[0x19, 0x23, 0x3D, 0x3B, 0x37, 0x3F, 0x29], XOR_KEY))
                .join(xdecode(&[0x0F, 0x37, 0x3B, 0x37, 0x2F, 0x29, 0x2F, 0x37, 0x3F], XOR_KEY))
                .join("master")
                .join("master.mdb"),
        );
    }

    for steam_root in steam_roots() {
        push_target_candidates(&steam_root, &mut push_candidate);
    }

    candidates
}

fn steam_roots() -> Vec<PathBuf> {
    let mut seen = BTreeSet::new();
    let mut roots = Vec::new();

    let base_roots = if let Some(home) = dirs::home_dir() {
        #[allow(unused_mut)]
        let mut roots = vec![home.join(".local/share/Steam"), home.join(".steam/steam")];
        #[cfg(target_os = "windows")]
        {
            if let Ok(progfiles) = std::env::var("ProgramFiles(x86)") {
                roots.push(PathBuf::from(progfiles).join("Steam"));
            }
            if let Ok(progfiles) = std::env::var("ProgramFiles") {
                roots.push(PathBuf::from(progfiles).join("Steam"));
            }
            if let Ok(localappdata) = std::env::var("LOCALAPPDATA") {
                roots.push(PathBuf::from(localappdata).join("Steam"));
            }
        }
        roots
    } else {
        Vec::new()
    };

    for root in base_roots {
        if seen.insert(root.clone()) {
            roots.push(root);
        }
    }

    let initial_roots = roots.clone();
    for root in initial_roots {
        for extra in steam_library_roots(&root) {
            if seen.insert(extra.clone()) {
                roots.push(extra);
            }
        }
    }

    roots
}

fn push_target_candidates<F>(steam_root: &Path, push_candidate: &mut F)
where
    F: FnMut(PathBuf),
{
    let compatdata = steam_root.join("steamapps/compatdata");

    let target_game_dir = compatdata.join(TARGET_APP_ID);
    if target_game_dir.is_dir() {
        push_candidate(
            target_game_dir
                .join("pfx/drive_c/users/steamuser")
                .join(master_db_relative_path()),
        );

        let users_root = target_game_dir.join("pfx/drive_c/users");
        if let Ok(users) = fs::read_dir(users_root) {
            for user_entry in users.flatten() {
                let candidate = user_entry.path().join(master_db_relative_path());
                push_candidate(candidate);
            }
        }
    }

    let entries = match fs::read_dir(&compatdata) {
        Ok(entries) => entries,
        Err(_) => return,
    };

    for entry in entries.flatten() {
        let game_dir = entry.path();
        let direct_steamuser = game_dir
            .join("pfx/drive_c/users/steamuser")
            .join(master_db_relative_path());
        push_candidate(direct_steamuser);

        let users_root = game_dir.join("pfx/drive_c/users");
        let users = match fs::read_dir(users_root) {
            Ok(users) => users,
            Err(_) => continue,
        };

        for user_entry in users.flatten() {
            let candidate = user_entry.path().join(master_db_relative_path());
            push_candidate(candidate);
        }
    }
}

fn steam_library_roots(steam_root: &Path) -> Vec<PathBuf> {
    let mut roots = Vec::new();
    let library_file = steam_root.join("steamapps/libraryfolders.vdf");
    let content = match fs::read_to_string(&library_file) {
        Ok(content) => content,
        Err(_) => return roots,
    };

    for line in content.lines() {
        let trimmed = line.trim();
        if !trimmed.contains("\"path\"") {
            continue;
        }

        if let Some(path) = trimmed.split('"').nth(3) {
            let path = path.trim();
            if !path.is_empty() {
                roots.push(PathBuf::from(path));
            }
        }
    }

    roots
}
