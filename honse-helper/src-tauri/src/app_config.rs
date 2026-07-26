use std::path::PathBuf;
use std::sync::{LazyLock, RwLock};

use shared::app_config::{AppConfig, MasterDbConfig};
use shared::worker_state::WorkerConfig;

const CONFIG_FILE_NAME: &str = "config.json";

fn config_path() -> Option<PathBuf> {
    let dir = dirs::data_dir()?.join("honse-helper");
    std::fs::create_dir_all(&dir).ok()?;
    Some(dir.join(CONFIG_FILE_NAME))
}

fn load_config() -> AppConfig {
    let path = match config_path() {
        Some(p) => p,
        None => return AppConfig::default(),
    };

    if path.exists() {
        if let Ok(content) = std::fs::read_to_string(&path) {
            if let Ok(config) = serde_json::from_str::<AppConfig>(&content) {
                return config;
            }
        }
    }

    AppConfig::default()
}

fn save_config(config: &AppConfig) {
    if let Some(path) = config_path() {
        if let Ok(json) = serde_json::to_string_pretty(config) {
            let _ = std::fs::write(path, json);
        }
    }
}

static APP_CONFIG: LazyLock<RwLock<AppConfig>> = LazyLock::new(|| RwLock::new(load_config()));

pub fn win_saddle_version() -> u8 {
    APP_CONFIG.read().unwrap().win_saddle_version
}

pub fn master_db_config() -> MasterDbConfig {
    APP_CONFIG.read().unwrap().master_db.clone()
}

pub fn worker_config() -> WorkerConfig {
    APP_CONFIG.read().unwrap().worker.clone()
}

pub fn update_master_db(path: &str, source: &str) {
    let mut config = APP_CONFIG.write().unwrap();
    config.master_db = MasterDbConfig {
        path: Some(path.to_string()),
        source: Some(source.to_string()),
    };
    save_config(&config);
}

pub fn update_worker_config(worker: &WorkerConfig) {
    let mut config = APP_CONFIG.write().unwrap();
    config.worker = worker.clone();
    save_config(&config);
}
