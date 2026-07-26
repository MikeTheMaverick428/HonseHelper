use serde::{Deserialize, Serialize};

use crate::worker_state::WorkerConfig;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MasterDbConfig {
    pub path: Option<String>,
    pub source: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    #[serde(default)]
    pub master_db: MasterDbConfig,
    #[serde(default)]
    pub worker: WorkerConfig,
    #[serde(default = "default_win_saddle_version")]
    pub win_saddle_version: u8,
}

fn default_win_saddle_version() -> u8 {
    1
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            master_db: MasterDbConfig::default(),
            worker: WorkerConfig::default(),
            win_saddle_version: 1,
        }
    }
}
