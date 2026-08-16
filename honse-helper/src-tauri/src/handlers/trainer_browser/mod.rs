use crate::db::app_db;
use crate::storage::affinity::{calculate_browser_affinity, AffinityStorage};
use crate::storage::trainers::query_trainers_page;
use crate::storage::veterans::VeteranStore;
use rusqlite::Connection;
use shared::{
    models::PaginationResponse,
    trainer_browser::{
        TrainerBrowserQuery, TrainerFilterOptions, TrainerPageItem,
    },
};
use std::collections::HashMap;
use std::sync::Mutex;
use tauri::{AppHandle, Manager, State, WebviewUrl, WebviewWindowBuilder};

pub mod uma_moe_api;

pub struct TrainerBrowserConfig {
    pub modes: Mutex<HashMap<String, String>>,
}

const LABEL: &str = "trainer-browser";

// ── Helpers ─────────────────────────────────────────────────────────

pub(crate) fn get_id_name_pairs(
    conn: &Connection,
    table: &str,
    where_clause: Option<&str>,
) -> Result<Vec<(i64, String)>, String> {
    let mut sql = format!("SELECT id, name FROM {} ", table);
    if let Some(where_clause) = where_clause {
        sql += where_clause;
    }
    sql += " ORDER BY name";
    let mut stmt = conn
        .prepare(&sql)
        .map_err(|e| format!("prepare {table} failed: {e}"))?;
    let mapped = stmt
        .query_map([], |row| {
            Ok((row.get::<_, i64>(0)?, row.get::<_, String>(1)?))
        })
        .map_err(|e| format!("query {table} failed: {e}"))?;
    mapped
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("collect {table} failed: {e}"))
}

fn get_id_name_pairs_distinct(
    conn: &Connection,
    table: &str,
    id_col: &str,
) -> Result<Vec<(i64, String)>, String> {
    let sql = format!(
        "SELECT DISTINCT {}, CAST({} AS TEXT) AS name FROM {} ORDER BY {}",
        id_col, id_col, table, id_col
    );
    let mut stmt = conn
        .prepare(&sql)
        .map_err(|e| format!("prepare {table} failed: {e}"))?;
    let mapped = stmt
        .query_map([], |row| {
            Ok((row.get::<_, i64>(0)?, row.get::<_, String>(1)?))
        })
        .map_err(|e| format!("query {table} failed: {e}"))?;
    mapped
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("collect {table} failed: {e}"))
}

// ── Filter Options ─────────────────────────────────────────────────

#[tauri::command]
pub fn get_trainer_filter_options() -> Result<TrainerFilterOptions, String> {
    let conn = app_db::open_app_database_connection()?;

    let characters = get_id_name_pairs(&conn, "character_data", Some("WHERE trainee = 1"))?;
    let card_types = get_id_name_pairs_distinct(&conn, "support_card_data", "card_type")?;
    let rarities = get_id_name_pairs_distinct(&conn, "support_card_data", "rarity")?;

    Ok(TrainerFilterOptions {
        characters,
        card_types,
        rarities,
    })
}

// ── Window Management ──────────────────────────────────────────────

#[tauri::command]
pub async fn open_trainer_browser(
    app: AppHandle,
    config: State<'_, TrainerBrowserConfig>,
    mode: Option<String>,
) -> Result<(), String> {
    {
        let mut modes = config.modes.lock().map_err(|e| e.to_string())?;
        if let Some(m) = &mode {
            modes.insert(LABEL.to_string(), m.clone());
        } else {
            modes.remove(LABEL);
        }
    }

    if let Some(win) = app.get_webview_window(LABEL) {
        let _ = win.set_focus();
        let _ = win.eval("window.location.reload()");
        return Ok(());
    }

    let data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?
        .join(LABEL);
    WebviewWindowBuilder::new(&app, LABEL, WebviewUrl::App("index.html".into()))
        .title("Trainer Browser")
        .inner_size(1200.0, 800.0)
        .resizable(true)
        .data_directory(data_dir)
        .build()
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn get_trainer_browser_mode(
    config: State<'_, TrainerBrowserConfig>,
    window_label: String,
) -> Result<Option<String>, String> {
    Ok(config
        .modes
        .lock()
        .map_err(|e| e.to_string())?
        .get(&window_label)
        .cloned())
}

// ── Query ──────────────────────────────────────────────────────────

#[tauri::command]
pub fn query_trainer_page(
    query: TrainerBrowserQuery,
    veteran_store: State<'_, Mutex<VeteranStore>>,
    affinity_store: State<'_, Mutex<AffinityStorage>>,
) -> Result<PaginationResponse<TrainerPageItem>, String> {
    let conn = app_db::open_app_database_connection()?;
    let mut response = query_trainers_page(&conn, &query)?;

    let mut store = veteran_store.lock().map_err(|e| e.to_string())?;
    let affinity = affinity_store.lock().map_err(|e| e.to_string())?;

    let hashes: Vec<u64> = response
        .results
        .iter()
        .filter_map(|t| t.borrow_veteran.as_ref().map(|v| v.hash.as_u64()))
        .collect();
    if !hashes.is_empty() {
        store.ensure_loaded(&conn, hashes.iter().copied())?;
        for item in &mut response.results {
            if let Some(v) = &mut item.borrow_veteran {
                if let Some(group) = store.get_veteran_group(v.hash.as_u64()) {
                    let result = calculate_browser_affinity(None, &affinity, &store, group);
                    v.affinity = Some(result.total_result());
                }
            }
        }
    }

    Ok(response)
}
