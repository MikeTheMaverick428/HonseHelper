#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app_config;
mod data_sync;
mod db;
mod external;
mod handlers;
mod resource_monitor;
mod storage;
mod veterans;
mod worker;

use db::master_db::MasterDbState;
use handlers::api_config::ApiKeyState;
use handlers::legacy_planner::LegacyPlannerStateHandle;
use handlers::race_dump::RaceDumpDetailState;
use handlers::support_card_browser::SupportCardBrowserConfig;
use handlers::trainee_browser::TraineeBrowserConfig;
use handlers::veteran_browser::BrowserConfig;
use handlers::worker::WorkerStatusState;
use std::collections::HashMap;
use std::sync::Mutex;
use storage::affinity::AffinityStorage;
use storage::veterans::VeteranStore;
use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};
use veterans::uma_moe_cache::UmaMoeCache;
use worker::WorkerState;

#[tauri::command]
async fn open_db_status_window(app: AppHandle) -> Result<(), String> {
    let label = "db-status";
    if let Some(win) = app.get_webview_window(label) {
        let _ = win.set_focus();
        return Ok(());
    }
    let data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?
        .join(label);
    WebviewWindowBuilder::new(&app, label, WebviewUrl::App("index.html".into()))
        .title("DB Status")
        .inner_size(720.0, 560.0)
        .resizable(true)
        .data_directory(data_dir)
        .build()
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
async fn open_worker_status_window(app: AppHandle) -> Result<(), String> {
    let label = "worker-status";
    if let Some(win) = app.get_webview_window(label) {
        let _ = win.set_focus();
        return Ok(());
    }
    let data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?
        .join(label);
    WebviewWindowBuilder::new(&app, label, WebviewUrl::App("index.html".into()))
        .title("Worker Status")
        .inner_size(720.0, 560.0)
        .resizable(true)
        .data_directory(data_dir)
        .build()
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
async fn open_race_dump_window(app: AppHandle) -> Result<(), String> {
    let label = "race-dump";
    if let Some(win) = app.get_webview_window(label) {
        let _ = win.set_focus();
        return Ok(());
    }
    let data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?
        .join(label);
    WebviewWindowBuilder::new(&app, label, WebviewUrl::App("index.html".into()))
        .title("Race Dumps")
        .inner_size(1200.0, 800.0)
        .resizable(true)
        .data_directory(data_dir)
        .build()
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
async fn open_dev_tools_window(app: AppHandle) -> Result<(), String> {
    if !shared::DEV_VIEW {
        return Err("Dev Tools is disabled. Enable shared::DEV_VIEW to use this.".to_string());
    }
    let label = "dev-tools";
    if let Some(win) = app.get_webview_window(label) {
        let _ = win.set_focus();
        return Ok(());
    }
    let data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?
        .join(label);
    WebviewWindowBuilder::new(&app, label, WebviewUrl::App("index.html".into()))
        .title("Dev Tools")
        .inner_size(900.0, 700.0)
        .resizable(true)
        .data_directory(data_dir)
        .build()
        .map_err(|e| e.to_string())?;
    resource_monitor::spawn_resource_monitor(app.clone());
    Ok(())
}

#[tauri::command]
async fn open_supplementary_data_window(app: AppHandle) -> Result<(), String> {
    let label = "supplementary-data";
    if let Some(win) = app.get_webview_window(label) {
        let _ = win.set_focus();
        return Ok(());
    }
    let data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?
        .join(label);
    WebviewWindowBuilder::new(&app, label, WebviewUrl::App("index.html".into()))
        .title("Supplementary Data")
        .inner_size(600.0, 500.0)
        .resizable(true)
        .data_directory(data_dir)
        .build()
        .map_err(|e| e.to_string())?;
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .manage(MasterDbState::new())
        .manage(WorkerState::new())
        .manage(WorkerStatusState::new())
        .manage(BrowserConfig {
            modes: Mutex::new(HashMap::new()),
            sources: Mutex::new(HashMap::new()),
            chosen_character_id: Mutex::new(None),
        })
        .manage(LegacyPlannerStateHandle {
            state: Mutex::new(None),
        })
        .manage(Mutex::new(VeteranStore::new()))
        .manage(Mutex::new(AffinityStorage::default()))
        .manage(RaceDumpDetailState(Mutex::new(None)))
        .manage(ApiKeyState::new())
        .manage(UmaMoeCache::new())
        .manage(SupportCardBrowserConfig {
            modes: Mutex::new(HashMap::new()),
        })
        .manage(TraineeBrowserConfig {
            modes: Mutex::new(HashMap::new()),
        })
        .setup(|app| {
            initialize_app_database();
            sync_app_database_in_background(app.handle().clone());

            if let Ok(conn) = db::app_db::open_app_database_connection() {
                if let Some(store) = app.try_state::<Mutex<VeteranStore>>() {
                    let _ = store.lock().unwrap().load_all(&conn);
                }
                if let Some(affinity) = app.try_state::<Mutex<AffinityStorage>>() {
                    let _ = affinity.lock().unwrap().load_all(&conn);
                }
            }

            handlers::worker::spawn_worker_supervisor(app.handle().clone());

            let handle = app.handle().clone();
            let ss = app.state::<WorkerStatusState>();
            let auto_start = ss.auto_start_enabled();
            if auto_start {
                std::thread::spawn(move || {
                    std::thread::sleep(std::time::Duration::from_millis(1000));
                    if let Err(e) = handlers::start_worker_inner(&handle) {
                        eprintln!("auto-start worker failed: {e}");
                    }
                });
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            db::master_db::discover_master_db_path,
            db::master_db::get_master_db_status,
            db::master_db::set_master_db_path,
            db::app_db::sync_app_database,
            db::app_db::get_app_db_status,
            open_db_status_window,
            open_worker_status_window,
            handlers::api_config::open_api_config_window,
            open_race_dump_window,
            open_supplementary_data_window,
            open_dev_tools_window,
            handlers::start_worker,
            handlers::send_worker_request,
            handlers::save_worker_response,
            handlers::veterans::gather_veterans,
            handlers::veterans::export_veterans_to_json,
            handlers::race_dump::save_race_dump,
            handlers::race_dump::get_race_dumps,
            handlers::race_dump::delete_race_dump,
            handlers::race_dump::get_race_dump_detail,
            handlers::race_dump::export_race_dump_hakuraku,
            handlers::race_dump::export_race_dumps_batch,
            handlers::race_dump::open_race_dump_detail_window,
            handlers::race_dump::query_race_dump_page,
            handlers::race_dump::get_race_dump_filter_options,
            handlers::race_dump::return_race_dump_selection,
            handlers::race_dump::emit_race_dump_tags_changed,
            handlers::stop_worker,
            handlers::is_worker_running,
            handlers::trophy_data::import_trophy_data,
            handlers::card_data::import_card_data,
            handlers::card_data::get_last_gather_time,
            handlers::support_card_data::import_support_card_data,
            handlers::supplementary_data::get_supplementary_data_status,
            handlers::supplementary_data::check_supplementary_data_updates,
            handlers::supplementary_data::sync_supplementary_data,
            handlers::worker::get_worker_status,
            handlers::worker::set_worker_auto_start,
            handlers::worker::set_worker_retry_config,
            handlers::worker::set_worker_discovery_interval,
            handlers::worker::reset_worker_retry_count,
            handlers::api_config::get_api_key_status,
            handlers::api_config::set_api_key,
            handlers::veteran_browser::local::get_veteran_detail,
            handlers::veteran_browser::local::get_veteran_sparks,
            handlers::veteran_browser::local::get_veteran_wins,
            handlers::veteran_browser::local::get_veteran_parents,
            handlers::veteran_browser::local::get_parent_sparks,
            handlers::veteran_browser::local::get_parent_wins,
            handlers::veteran_browser::uma_moe_api::get_uma_moe_parent_sparks,
            handlers::veteran_browser::uma_moe_api::get_uma_moe_parent_wins,
            handlers::veteran_browser::save_preset,
            handlers::veteran_browser::load_preset_active,
            handlers::veteran_browser::load_preset,
            handlers::veteran_browser::delete_preset,
            handlers::veteran_browser::rename_preset,
            handlers::veteran_browser::list_presets,
            handlers::veteran_browser::get_filter_options,
            handlers::veteran_browser::open_veteran_browser,
            handlers::veteran_browser::return_veteran_selection,
            handlers::veteran_browser::get_browser_mode,
            handlers::veteran_browser::get_browser_source,
            handlers::veteran_browser::local::query_veteran_store_page,
            handlers::veteran_browser::local::delete_veteran,
            handlers::veteran_browser::local::get_veteran_skills,
            handlers::veteran_browser::local::get_veteran_support_cards,
            handlers::veteran_browser::local::get_skill_detail,
            handlers::veteran_browser::uma_moe_api::query_uma_moe_veterans,
            handlers::veteran_browser::uma_moe_api::save_uma_moe_veteran,
            handlers::veteran_browser::uma_moe_api::get_uma_moe_veteran_detail,
            handlers::legacy_planner::set_legacy_planner_slot_uma_moe_veteran,
            handlers::legacy_planner::get_legacy_planner_state,
            handlers::legacy_planner::set_legacy_planner_chosen,
            handlers::legacy_planner::clear_legacy_planner_chosen,
            handlers::legacy_planner::set_legacy_planner_slot_veteran,
            handlers::legacy_planner::set_legacy_planner_slot_character,
            handlers::legacy_planner::clear_legacy_planner_slot,
            handlers::legacy_planner::clear_legacy_planner,
            handlers::legacy_planner::compute_veteran_affinities,
            handlers::legacy_planner::get_planner_trainee_characters,
            handlers::legacy_planner::open_legacy_planner_window,
            handlers::legacy_planner::compute_planner_affinities,
            handlers::legacy_planner::get_trainee_stats,
            handlers::legacy_planner::get_trainee_available_rarities,
            handlers::legacy_planner::get_planner_affinity_summary,
            handlers::legacy_planner::get_planner_spark_summary,
            handlers::legacy_planner::get_planner_inspiration_summary,
            handlers::tags::search_tags,
            handlers::tags::add_tag,
            handlers::tags::get_veteran_tags,
            handlers::tags::tag_veteran,
            handlers::tags::untag_veteran,
            handlers::tags::get_race_dump_tags,
            handlers::tags::tag_race_dump,
            handlers::tags::untag_race_dump,
            handlers::tags::get_all_tags,
            handlers::support_card_browser::open_support_card_browser,
            handlers::support_card_browser::get_support_card_browser_mode,
            handlers::support_card_browser::query_support_card_store_page,
            handlers::support_card_browser::get_support_card_filter_options,
            handlers::support_card_browser::save_support_card_preset,
            handlers::support_card_browser::load_support_card_preset_active,
            handlers::support_card_browser::load_support_card_preset,
            handlers::support_card_browser::delete_support_card_preset,
            handlers::support_card_browser::list_support_card_presets,
            handlers::support_card_browser::get_support_card_detail,
            handlers::trainee_browser::open_trainee_browser,
            handlers::trainee_browser::get_trainee_browser_mode,
            handlers::trainee_browser::query_trainee_cards,
            handlers::trainee_browser::get_trainee_detail,
            handlers::trainee_browser::get_trainee_filter_options,
            handlers::trainee_browser::save_trainee_preset,
            handlers::trainee_browser::load_trainee_preset_active,
            handlers::trainee_browser::load_trainee_preset,
            handlers::trainee_browser::delete_trainee_preset,
            handlers::trainee_browser::list_trainee_presets,
            handlers::trainee_browser::return_trainee_selection,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn initialize_app_database() {
    if let Err(err) = db::app_db::ensure_app_db_schema() {
        eprintln!("failed to initialize app DB schema: {err}");
    }
}

fn sync_app_database_in_background(app: AppHandle) {
    std::thread::spawn(move || {
        let state = app.state::<MasterDbState>();
        let discovery = db::master_db::discover_master_db_path_impl(&app, &state);
        if discovery
            .as_ref()
            .ok()
            .and_then(|status| status.path.as_ref())
            .is_some()
        {
            let _ = db::app_db::sync_app_database_from_state(&app, &state);
        }
    });
}
