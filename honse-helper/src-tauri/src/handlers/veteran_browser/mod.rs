use crate::db::app_db;
use chrono::Utc;
use rusqlite::{params, Connection, OptionalExtension};
use serde_json::json;
use shared::veteran_browser::*;
use std::collections::HashMap;
use std::sync::Mutex;
use tauri::{AppHandle, Emitter, Manager, State, WebviewUrl, WebviewWindowBuilder};

pub mod local;
pub mod uma_moe_api;

pub struct BrowserConfig {
    pub modes: Mutex<HashMap<String, String>>,
    pub sources: Mutex<HashMap<String, String>>,
    pub chosen_character_id: Mutex<Option<i64>>,
}

const DEFAULT_BROWSER_TYPE: &str = "veteran";

// ── Presets ────────────────────────────────────────────────────────

#[tauri::command]
pub fn save_preset(
    name: String,
    filters: Option<String>,
    sort: Option<String>,
    browser_type: Option<String>,
) -> Result<(), String> {
    let bt = browser_type.unwrap_or_else(|| DEFAULT_BROWSER_TYPE.into());
    let conn = app_db::open_app_database_connection()?;
    let now = Utc::now().to_rfc3339();

    // Upsert the preset row, only setting created_at and active
    conn.execute(
        "INSERT INTO browser_presets (browser_type, name, filters, sort, created_at, active) VALUES (?1, ?2, ?3, ?4, ?5, 1) \
         ON CONFLICT(browser_type, name) DO UPDATE SET created_at = excluded.created_at, active = 1",
        params![bt, name, filters, sort, now],
    )
    .map_err(|e| format!("save preset failed: {e}"))?;

    // Only update filters/sort if provided (non-None)
    if let Some(ref f) = filters {
        conn.execute(
            "UPDATE browser_presets SET filters = ?1 WHERE browser_type = ?2 AND name = ?3",
            params![f, bt, name],
        )
        .map_err(|e| format!("save preset filters failed: {e}"))?;
    }
    if let Some(ref s) = sort {
        conn.execute(
            "UPDATE browser_presets SET sort = ?1 WHERE browser_type = ?2 AND name = ?3",
            params![s, bt, name],
        )
        .map_err(|e| format!("save preset sort failed: {e}"))?;
    }

    conn.execute(
        "UPDATE browser_presets SET active = NULL WHERE browser_type = ?1 AND name != ?2",
        params![bt, name],
    )
    .map_err(|_| "deactivate preset failed")?;
    Ok(())
}

#[tauri::command]
pub fn load_preset_active(browser_type: Option<String>) -> Result<Option<PresetData>, String> {
    let bt = browser_type.unwrap_or_else(|| DEFAULT_BROWSER_TYPE.into());
    let conn = app_db::open_app_database_connection()?;
    let mut stmt = conn
        .prepare("SELECT name, filters, sort, created_at FROM browser_presets WHERE browser_type = ?1 AND active = 1")
        .map_err(|e| format!("load preset prepare failed: {e}"))?;
    let data = stmt
        .query_row(params![bt], |row| {
            Ok(PresetData {
                name: row.get(0)?,
                filters: row.get(1)?,
                sort: row.get(2)?,
                created_at: row.get(3)?,
            })
        })
        .optional()
        .map_err(|e| format!("load preset query failed: {e}"))?;
    Ok(data)
}

#[tauri::command]
pub fn load_preset(
    name: String,
    browser_type: Option<String>,
) -> Result<Option<PresetData>, String> {
    let bt = browser_type.unwrap_or_else(|| DEFAULT_BROWSER_TYPE.into());
    let conn = app_db::open_app_database_connection()?;
    let result = {
        let mut stmt = conn
            .prepare("SELECT name, filters, sort, created_at FROM browser_presets WHERE browser_type = ?1 AND name = ?2")
            .map_err(|e| format!("load preset prepare failed: {e}"))?;
        let data = stmt
            .query_row(params![bt, name], |row| {
                Ok(PresetData {
                    name: row.get(0)?,
                    filters: row.get(1)?,
                    sort: row.get(2)?,
                    created_at: row.get(3)?,
                })
            })
            .optional()
            .map_err(|e| format!("load preset query failed: {e}"))?;

        if data.is_some() {
            conn.execute(
                "UPDATE browser_presets SET active = NULL WHERE browser_type = ?1 AND name != ?2",
                params![bt, name],
            )
            .map_err(|_| "deactivate preset failed")?;
            conn.execute(
                "UPDATE browser_presets SET active = 1 WHERE browser_type = ?1 AND name = ?2",
                params![bt, name],
            )
            .map_err(|_| "activate preset failed")?;
        }

        data
    };
    Ok(result)
}

#[tauri::command]
pub fn delete_preset(name: String, browser_type: Option<String>) -> Result<(), String> {
    let bt = browser_type.unwrap_or_else(|| DEFAULT_BROWSER_TYPE.into());
    let conn = app_db::open_app_database_connection()?;
    conn.execute(
        "DELETE FROM browser_presets WHERE browser_type = ?1 AND name = ?2",
        params![bt, name],
    )
    .map_err(|e| format!("delete preset failed: {e}"))?;
    Ok(())
}

#[tauri::command]
pub fn rename_preset(
    name: String,
    new_name: String,
    browser_type: Option<String>,
) -> Result<(), String> {
    let bt = browser_type.unwrap_or_else(|| DEFAULT_BROWSER_TYPE.into());
    let conn = app_db::open_app_database_connection()?;
    let now = Utc::now().to_rfc3339();
    let affected = conn
        .execute(
            "UPDATE browser_presets SET name = ?, created_at = ? WHERE browser_type = ? AND name = ?",
            params![new_name, now, bt, name],
        )
        .map_err(|e| format!("rename preset failed: {e}"))?;
    if affected == 0 {
        return Err(format!("preset '{}' not found", name));
    }
    Ok(())
}

#[tauri::command]
pub fn list_presets(
    browser_type: Option<String>,
    preset_type: Option<String>,
) -> Result<Vec<String>, String> {
    let bt = browser_type.unwrap_or_else(|| DEFAULT_BROWSER_TYPE.into());
    let conn = app_db::open_app_database_connection()?;
    let sql = match preset_type.as_deref() {
        Some("filter") => {
            "SELECT name FROM browser_presets WHERE browser_type = ?1 AND name != '__active__' AND filters IS NOT NULL ORDER BY created_at DESC"
        }
        Some("sort") => {
            "SELECT name FROM browser_presets WHERE browser_type = ?1 AND name != '__active__' AND sort IS NOT NULL ORDER BY created_at DESC"
        }
        _ => {
            "SELECT name FROM browser_presets WHERE browser_type = ?1 AND name != '__active__' ORDER BY created_at DESC"
        }
    };
    let presets = {
        let mut stmt = conn
            .prepare(sql)
            .map_err(|e| format!("list presets prepare failed: {e}"))?;
        let mapped = stmt
            .query_map(params![bt], |row| row.get::<_, String>(0))
            .map_err(|e| format!("list presets query failed: {e}"))?;
        mapped
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| format!("list presets collect failed: {e}"))?
    };
    Ok(presets)
}

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
    where_clause: Option<&str>,
) -> Result<Vec<(i64, String)>, String> {
    let mut sql = format!("SELECT DISTINCT {}, name FROM {} ", id_col, table);
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

// ── Filter Options ─────────────────────────────────────────────────

#[tauri::command]
pub fn get_filter_options() -> Result<FilterOptions, String> {
    let conn = app_db::open_app_database_connection()?;

    let characters = get_id_name_pairs(&conn, "character_data", Some("WHERE trainee = 1"))?;
    let trainees = get_id_name_pairs(&conn, "trainee_data", None)?;
    let blue_spark_groups = get_id_name_pairs_distinct(
        &conn,
        "spark_data",
        "group_id",
        Some("WHERE spark_type = 1"),
    )?;
    let pink_spark_groups = get_id_name_pairs_distinct(
        &conn,
        "spark_data",
        "group_id",
        Some("WHERE spark_type = 2"),
    )?;
    let green_spark_groups = get_id_name_pairs_distinct(
        &conn,
        "spark_data",
        "group_id",
        Some("WHERE spark_type = 3"),
    )?;
    let white_spark_groups = get_id_name_pairs_distinct(
        &conn,
        "spark_data",
        "group_id",
        Some("WHERE spark_type IN (4,5,6)"),
    )?;

    let scenarios = get_id_name_pairs(&conn, "scenario_data", None)?;

    let tags: Vec<String> = {
        let mut stmt = conn
            .prepare("SELECT DISTINCT t.tag_value FROM tag t JOIN veteran_has_tag vht ON vht.tag_id = t.id ORDER BY t.tag_value")
            .map_err(|e| format!("prepare tags query failed: {e}"))?;
        let rows = stmt
            .query_map([], |row| row.get::<_, String>(0))
            .map_err(|e| format!("query tags failed: {e}"))?;
        let mut result = Vec::new();
        for row in rows {
            result.push(row.map_err(|e| format!("tag row error: {e}"))?);
        }
        result
    };

    Ok(FilterOptions {
        characters,
        trainees,
        blue_spark_groups,
        pink_spark_groups,
        green_spark_groups,
        white_spark_groups,
        scenarios,
        tags,
    })
}

// ── Window Management ──────────────────────────────────────────────

#[tauri::command]
pub async fn open_veteran_browser(
    app: AppHandle,
    config: State<'_, BrowserConfig>,
    mode: Option<String>,
    source: Option<String>,
    chosen_character_id: Option<i64>,
) -> Result<(), String> {
    let label = "veteran-browser";

    // Always update state first so existing windows pick up new source/mode
    {
        let mut modes = config.modes.lock().map_err(|e| e.to_string())?;
        if let Some(m) = &mode {
            modes.insert(label.to_string(), m.clone());
        } else {
            modes.remove(label);
        }
    }
    {
        let mut sources = config.sources.lock().map_err(|e| e.to_string())?;
        if let Some(s) = &source {
            sources.insert(label.to_string(), s.clone());
        } else {
            sources.remove(label);
        }
    }
    *config
        .chosen_character_id
        .lock()
        .map_err(|e| e.to_string())? = chosen_character_id;

    if let Some(win) = app.get_webview_window(label) {
        let _ = win.set_focus();
        let _ = win.eval("window.location.reload()");
        return Ok(());
    }

    let data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?
        .join(label);
    WebviewWindowBuilder::new(&app, label, WebviewUrl::App("index.html".into()))
        .title("Veteran Browser")
        .inner_size(1200.0, 800.0)
        .resizable(true)
        .data_directory(data_dir)
        .build()
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn get_browser_mode(
    config: State<'_, BrowserConfig>,
    window_label: String,
) -> Result<Option<String>, String> {
    Ok(config
        .modes
        .lock()
        .map_err(|e| e.to_string())?
        .get(&window_label)
        .cloned())
}

#[tauri::command]
pub fn get_browser_source(
    config: State<'_, BrowserConfig>,
    window_label: String,
) -> Result<Option<String>, String> {
    Ok(config
        .sources
        .lock()
        .map_err(|e| e.to_string())?
        .get(&window_label)
        .cloned())
}

#[tauri::command]
pub fn return_veteran_selection(
    app: AppHandle,
    config: State<'_, BrowserConfig>,
    hash: String,
) -> Result<(), String> {
    let config_lock = config.modes.lock().map_err(|e| e.to_string())?;
    let slot_label = config_lock
        .get("veteran-browser")
        .cloned()
        .and_then(|mode| mode.strip_prefix("select_veteran:").map(|s| s.to_string()));
    drop(config_lock);

    let source = config
        .sources
        .lock()
        .map_err(|e| e.to_string())?
        .get("veteran-browser")
        .cloned()
        .unwrap_or_else(|| "local".to_string());

    let payload = json!({
        "hash": hash,
        "slot_label": slot_label,
        "source": source,
    });
    let _ = app.emit("veteran-selected", payload);
    if let Some(win) = app.get_webview_window("veteran-browser") {
        let _ = win.close();
    }
    Ok(())
}
