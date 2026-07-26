use crate::data_sync;
use crate::data_sync::download::{download_zip, extract_json_from_zip};
use crate::db::app_db;
use chrono::Utc;
use rusqlite::{params, Connection, OptionalExtension};
use serde::Deserialize;
use shared::{
    DatasetCheckEntry, DatasetSyncStatus, SupplementaryDataCheckReport, SupplementaryDataSyncReport,
};
use std::collections::HashMap;
use tauri::{AppHandle, Emitter};

const SUPPLEMENTARY_DATA_EVENT: &str = "supplementary-data-sync-status";

// --- JSON deserialization types ---

#[derive(Debug, Deserialize)]
struct EventsEntry {
    #[serde(default)]
    chain_events: Vec<Event>,
    #[serde(default)]
    random_events: Vec<Event>,
}

#[derive(Debug, Deserialize)]
struct Event {
    #[serde(default)]
    support_card_id: Option<i64>,
    #[serde(default)]
    character_id: Option<i64>,
    #[serde(default)]
    trainee_id: Option<i64>,
    #[serde(default)]
    id: Option<i64>,
    name: String,
    category: String,
    choices: Vec<Choice>,
    #[serde(default)]
    conditions: Option<Vec<Vec<serde_json::Value>>>,
}

#[derive(Debug, Deserialize)]
struct Branch {
    #[serde(default)]
    probability: Option<String>,
    rewards: Vec<Reward>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    branches: Vec<Branch>,
}

#[derive(Debug, Deserialize)]
struct Reward {
    #[serde(rename = "type")]
    reward_type: i64,
    #[serde(default)]
    size: Option<i64>,
    #[serde(default)]
    skill_id: Option<i64>,
    #[serde(default)]
    negative: bool,
    #[serde(default)]
    alternatives: Vec<i64>,
    #[serde(default)]
    effect_id: Option<i64>,
}

// --- Metadata helpers ---

fn read_metadata(conn: &Connection, key: &str) -> Result<Option<String>, String> {
    conn.query_row(
        "SELECT value FROM db_metadata WHERE key = ?1",
        params![key],
        |row| row.get(0),
    )
    .optional()
    .map_err(|e| e.to_string())
}

fn write_metadata(conn: &Connection, key: &str, value: &str) -> Result<(), String> {
    let now = Utc::now().to_rfc3339();
    conn.execute(
        "INSERT INTO db_metadata (key, value, created_at) VALUES (?1, ?2, ?3)
         ON CONFLICT(key) DO UPDATE SET value = excluded.value, created_at = excluded.created_at",
        params![key, value, now],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

fn get_base_url(conn: &Connection) -> Result<String, String> {
    let stored = read_metadata(conn, "supp_data_base_url")?;
    Ok(stored.unwrap_or_else(|| data_sync::DEFAULT_BASE_URL.to_string()))
}

fn read_dataset_version(conn: &Connection, dataset_id: &str) -> Result<Option<i64>, String> {
    let key = format!("supp_data_{dataset_id}_version");
    match read_metadata(conn, &key)? {
        Some(v) => v.parse::<i64>().map(Some).map_err(|e| e.to_string()),
        None => Ok(None),
    }
}

fn read_dataset_sha256(conn: &Connection, dataset_id: &str) -> Result<Option<String>, String> {
    let key = format!("supp_data_{dataset_id}_sha256");
    read_metadata(conn, &key)
}

fn write_dataset_metadata(
    conn: &Connection,
    dataset_id: &str,
    version: i64,
    sha256: &str,
) -> Result<(), String> {
    write_metadata(
        conn,
        &format!("supp_data_{dataset_id}_version"),
        &version.to_string(),
    )?;
    write_metadata(conn, &format!("supp_data_{dataset_id}_sha256"), sha256)?;
    write_metadata(conn, "supp_data_synced_at", &Utc::now().to_rfc3339())?;
    Ok(())
}

// --- Import logic ---

fn import_events_json_str(json_str: &str) -> Result<SupplementaryDataSyncReport, String> {
    let entries: HashMap<String, EventsEntry> =
        serde_json::from_str(json_str).map_err(|e| format!("failed to parse JSON: {e}"))?;

    let mut conn = app_db::open_app_database_connection().map_err(|e| e.to_string())?;
    let tx = conn
        .transaction()
        .map_err(|e| format!("transaction: {e}"))?;

    let mut event_stmt = tx
        .prepare(
            "INSERT OR IGNORE INTO support_event (story_id, support_card_id, character_id, trainee_id, event_name, category, conditions) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        )
        .map_err(|e| e.to_string())?;
    let mut choice_stmt = tx
        .prepare("INSERT INTO support_event_choice (story_id, choice_index) VALUES (?1, ?2)")
        .map_err(|e| e.to_string())?;
    let mut branch_stmt = tx
        .prepare("INSERT INTO support_event_branch (choice_id, branch_index, probability) VALUES (?1, ?2, ?3)")
        .map_err(|e| e.to_string())?;
    let mut reward_stmt = tx
        .prepare(
            "INSERT INTO support_event_reward (choice_id, branch_id, reward_type, size, skill_id, negative, alternatives, effect_id, is_support_event, is_trainee_event) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
        )
        .map_err(|e| e.to_string())?;

    let mut event_count = 0_i64;
    let mut choice_count = 0_i64;
    let mut branch_count = 0_i64;
    let mut reward_count = 0_i64;

    for (_sc_id, entry) in &entries {
        for event in entry.chain_events.iter().chain(&entry.random_events) {
            let story_id = match event.id {
                Some(id) => id,
                None => continue,
            };

            let conditions_json = event
                .conditions
                .as_ref()
                .map(|c| serde_json::to_string(c).unwrap_or_default());

            let rows = event_stmt
                .execute(params![
                    story_id,
                    event.support_card_id,
                    event.character_id,
                    event.trainee_id,
                    event.name,
                    event.category,
                    conditions_json
                ])
                .map_err(|e| e.to_string())?;
            if rows == 0 {
                continue;
            }
            event_count += 1;

            for (idx, choice) in event.choices.iter().enumerate() {
                choice_count += 1;
                choice_stmt
                    .execute(params![story_id, idx as i64])
                    .map_err(|e| e.to_string())?;

                let choice_id = tx.last_insert_rowid();

                for (bi, branch) in choice.branches.iter().enumerate() {
                    branch_count += 1;
                    let probability = branch.probability.as_ref().map(|p| {
                        if p.starts_with('~') {
                            p.clone()
                        } else {
                            format!("~{}", p)
                        }
                    });
                    branch_stmt
                        .execute(params![choice_id, bi as i64, probability])
                        .map_err(|e| e.to_string())?;

                    let branch_id = tx.last_insert_rowid();

                    for reward in &branch.rewards {
                        reward_count += 1;
                        let alternatives_json = if reward.alternatives.is_empty() {
                            None
                        } else {
                            Some(
                                serde_json::to_string(&reward.alternatives)
                                    .map_err(|e| e.to_string())?,
                            )
                        };
                        reward_stmt
                            .execute(params![
                                choice_id,
                                branch_id,
                                reward.reward_type,
                                reward.size,
                                reward.skill_id,
                                reward.negative,
                                alternatives_json,
                                reward.effect_id,
                                event.support_card_id.is_some(),
                                event.trainee_id.is_some() || event.character_id.is_some(),
                            ])
                            .map_err(|e| e.to_string())?;
                    }
                }
            }
        }
    }

    drop(reward_stmt);
    drop(branch_stmt);
    drop(choice_stmt);
    drop(event_stmt);
    tx.commit().map_err(|e| e.to_string())?;

    let now = Utc::now().to_rfc3339();
    Ok(SupplementaryDataSyncReport {
        synced: true,
        up_to_date: true,
        event_count,
        choice_count,
        reward_count,
        datasets: vec![DatasetSyncStatus {
            id: "support-events".to_string(),
            version: 0,
            event_count,
            choice_count,
            reward_count,
        }],
        synced_at: now,
        message: format!(
            "imported {event_count} events, {choice_count} choices, {branch_count} branches, {reward_count} rewards"
        ),
    })
}

// --- Build status report from current DB state ---

fn build_status_report(conn: &Connection) -> Result<Option<SupplementaryDataSyncReport>, String> {
    let exists: bool = conn
        .query_row(
            "SELECT 1 FROM sqlite_master WHERE type='table' AND name='support_event' LIMIT 1",
            [],
            |_| Ok(true),
        )
        .unwrap_or(false);

    if !exists {
        return Ok(None);
    }

    let event_count: i64 = conn
        .query_row("SELECT COUNT(*) FROM support_event", [], |r| r.get(0))
        .unwrap_or(0);
    let choice_count: i64 = conn
        .query_row("SELECT COUNT(*) FROM support_event_choice", [], |r| {
            r.get(0)
        })
        .unwrap_or(0);
    let reward_count: i64 = conn
        .query_row("SELECT COUNT(*) FROM support_event_reward", [], |r| {
            r.get(0)
        })
        .unwrap_or(0);

    let synced = event_count > 0;
    let version = read_dataset_version(conn, "support-events").ok().flatten();
    let synced_at = read_metadata(conn, "supp_data_synced_at")
        .ok()
        .flatten()
        .unwrap_or_default();

    Ok(Some(SupplementaryDataSyncReport {
        synced,
        up_to_date: synced,
        event_count,
        choice_count,
        reward_count,
        datasets: vec![DatasetSyncStatus {
            id: "support-events".to_string(),
            version: version.unwrap_or(0),
            event_count,
            choice_count,
            reward_count,
        }],
        synced_at,
        message: if synced {
            format!("{event_count} events, {choice_count} choices, {reward_count} rewards loaded")
        } else {
            "supplementary data not imported".to_string()
        },
    }))
}

// --- Tauri commands ---

#[tauri::command]
pub fn get_supplementary_data_status() -> Result<Option<SupplementaryDataSyncReport>, String> {
    let conn = app_db::open_app_database_connection().map_err(|e| e.to_string())?;
    build_status_report(&conn)
}

#[tauri::command]
pub async fn check_supplementary_data_updates() -> Result<SupplementaryDataCheckReport, String> {
    let conn = app_db::open_app_database_connection().map_err(|e| format!("open db: {e}"))?;
    let base_url = get_base_url(&conn)?;

    let manifest = data_sync::manifest::fetch_manifest(&base_url).await?;

    let mut datasets = Vec::new();
    for entry in &manifest.files {
        let local_version = read_dataset_version(&conn, &entry.id)?;
        let local_sha256 = read_dataset_sha256(&conn, &entry.id)?;
        datasets.push(DatasetCheckEntry {
            id: entry.id.clone(),
            available_version: entry.version,
            local_version,
            needs_update: local_sha256.as_deref() != Some(&entry.sha256_json),
        });
    }

    Ok(SupplementaryDataCheckReport { datasets })
}

#[tauri::command]
pub async fn sync_supplementary_data(
    app: AppHandle,
    dataset_ids: Vec<String>,
) -> Result<SupplementaryDataSyncReport, String> {
    let conn = app_db::open_app_database_connection().map_err(|e| format!("open db: {e}"))?;
    let base_url = get_base_url(&conn)?;

    let manifest = data_sync::manifest::fetch_manifest(&base_url).await?;

    {
        let tx_conn =
            app_db::open_app_database_connection().map_err(|e| format!("open db for setup: {e}"))?;
        tx_conn
            .execute_batch(app_db::SUPPORT_EVENT_DROP_SQL)
            .map_err(|e| format!("drop event tables: {e}"))?;
        tx_conn
            .execute_batch(app_db::SUPPORT_EVENT_CREATE_SQL)
            .map_err(|e| format!("create event tables: {e}"))?;
    }

    let mut all_events = 0_i64;
    let mut all_choices = 0_i64;
    let mut all_rewards = 0_i64;
    let mut synced_datasets = Vec::new();

    for id in &dataset_ids {
        let entry = manifest
            .files
            .iter()
            .find(|f| f.id == *id)
            .ok_or_else(|| format!("dataset '{id}' not found in remote manifest"))?;

        let url = format!("{}/{}", base_url.trim_end_matches('/'), entry.zip_filename);
        let zip_bytes = download_zip(&url, &entry.sha256_zip).await?;
        let json_str = extract_json_from_zip(&zip_bytes, &entry.filename)?;

        let report = import_events_json_str(&json_str)?;
        all_events += report.event_count;
        all_choices += report.choice_count;
        all_rewards += report.reward_count;

        if let Ok(meta_conn) = app_db::open_app_database_connection() {
            let _ =
                write_dataset_metadata(&meta_conn, &entry.id, entry.version, &entry.sha256_json);
        }

        synced_datasets.push(DatasetSyncStatus {
            id: entry.id.clone(),
            version: entry.version,
            event_count: report.event_count,
            choice_count: report.choice_count,
            reward_count: report.reward_count,
        });
    }

    let now = Utc::now().to_rfc3339();
    let report = SupplementaryDataSyncReport {
        synced: true,
        up_to_date: true,
        event_count: all_events,
        choice_count: all_choices,
        reward_count: all_rewards,
        datasets: synced_datasets,
        synced_at: now,
        message: format!(
            "synced {dataset_ids_len} dataset(s): {all_events} events, {all_choices} choices, {all_rewards} rewards",
            dataset_ids_len = dataset_ids.len(),
        ),
    };

    let _ = app.emit(SUPPLEMENTARY_DATA_EVENT, report.clone());
    Ok(report)
}
