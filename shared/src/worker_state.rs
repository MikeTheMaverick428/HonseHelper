use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkerStatusReport {
    pub worker_running: bool,
    pub process_found: bool,
    pub process_name: Option<String>,
    pub process_path: Option<String>,
    pub current_known_view: Option<String>,
    pub current_view_id_raw: Option<i32>,
    pub current_view_kclass: Option<String>,
    pub current_view_class: Option<String>,
    pub current_view_ptr: Option<String>,
    pub current_scene_base_ptr: Option<String>,
    pub current_scene_class: Option<String>,
    pub last_known_view: Option<String>,
    pub auto_start: bool,
    pub retry_count: u32,
    pub max_retries: u32,
    pub retry_interval_secs: u32,
    pub discovery_interval_secs: u32,
}

impl Default for WorkerStatusReport {
    fn default() -> Self {
        Self {
            worker_running: false,
            process_found: false,
            process_name: None,
            process_path: None,
            current_known_view: None,
            current_view_id_raw: None,
            current_view_kclass: None,
            current_view_class: None,
            current_view_ptr: None,
            current_scene_base_ptr: None,
            current_scene_class: None,
            last_known_view: None,
            auto_start: true,
            retry_count: 0,
            max_retries: 10,
            retry_interval_secs: 30,
            discovery_interval_secs: 30,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkerConfig {
    pub auto_start: bool,
    pub max_retries: u32,
    pub retry_interval_secs: u32,
    pub discovery_interval_secs: u32,
}

impl Default for WorkerConfig {
    fn default() -> Self {
        Self {
            auto_start: true,
            max_retries: 10,
            retry_interval_secs: 30,
            discovery_interval_secs: 30,
        }
    }
}
