use crate::db::{app_db, schema::VeteranSchema};
use crate::worker::WorkerState;
use chrono::Utc;
use honse_worker::protocol::{
    parse_msgpack_frame_response, write_msgpack_request_framed, WorkerCommand, WorkerRequest,
};
use rusqlite::OptionalExtension;
use std::sync::mpsc::Receiver;
use std::time::Duration;
use tauri::{AppHandle, Manager, State};

const CARD_TIMEOUT_MS: u64 = 10_000;

#[tauri::command]
pub async fn import_card_data(
    app: AppHandle,
    mut request: WorkerRequest,
    timeout_ms: Option<u64>,
) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || {
        let state: State<'_, WorkerState> = app.state();

        if !matches!(request.command, WorkerCommand::GetCardData { .. }) {
            return Err("expected get_card_data command".to_string());
        }

        let request_id = request.id.unwrap_or_else(|| state.next_request_id());
        request.id = Some(request_id);

        let mut app_conn = app_db::open_app_database_connection()?;
        VeteranSchema::ensure_current(&app_conn)
            .map_err(|e| format!("schema init: {e}"))?;

        let receiver = state.register_pending(request_id)?;

        state.with_running_worker(|running| {
            write_msgpack_request_framed(&mut running.stdin, &request)
                .map_err(|e| format!("write failed: {e}"))
        })?;

        let frame = await_frame(
            &state,
            request_id,
            receiver,
            timeout_ms.unwrap_or(CARD_TIMEOUT_MS),
        )?;

        let response =
            parse_msgpack_frame_response(&frame).ok_or("failed to parse response")?;

        let payload = match response {
            honse_worker::protocol::WorkerResponse::Ok(ok) => super::rmpv_to_json(ok.payload),
            honse_worker::protocol::WorkerResponse::Err(err) => return Err(err.error),
            _ => return Err("unexpected response type".to_string()),
        };

        let card_dic = payload
            .get("card_dic")
            .and_then(|v| v.as_array())
            .ok_or("payload missing card_dic")?;

        let tx = app_conn
            .transaction()
            .map_err(|e| format!("transaction: {e}"))?;

        let mut stmt = tx
            .prepare(
                "INSERT OR REPLACE INTO trainee_owned (trainee_id, rarity, talent_level) VALUES (?1, ?2, ?3)",
            )
            .map_err(|e| format!("prepare: {e}"))?;

        for entry in card_dic {
            let tid = entry["card_id"]
                .as_i64()
                .ok_or("entry missing card_id")?;
            let rarity = entry["rarity"].as_i64().ok_or("entry missing rarity")?;
            let talent = entry["talent_level"]
                .as_i64()
                .ok_or("entry missing talent_level")?;
            stmt.execute(rusqlite::params![tid, rarity, talent])
                .map_err(|e| format!("insert: {e}"))?;
        }

        drop(stmt);
        tx.commit().map_err(|e| format!("commit: {e}"))?;

        // Import piece counts if present
        if let Some(piece_counts) = payload.get("piece_counts").and_then(|v| v.as_array()) {
            let tx2 = app_conn
                .transaction()
                .map_err(|e| format!("piece transaction: {e}"))?;

            let mut piece_stmt = tx2
                .prepare(
                    "INSERT OR REPLACE INTO piece_owned (trainee_id, shard_count) VALUES (?1, ?2)",
                )
                .map_err(|e| format!("piece prepare: {e}"))?;

            for entry in piece_counts {
                let tid = entry["_key"]
                    .as_i64()
                    .ok_or("piece entry missing _key")?;
                let shards = entry["shard_count"]
                    .as_i64()
                    .ok_or("piece entry missing shard_count")?;
                piece_stmt
                    .execute(rusqlite::params![tid, shards])
                    .map_err(|e| format!("piece insert: {e}"))?;
            }

            drop(piece_stmt);
            tx2.commit().map_err(|e| format!("piece commit: {e}"))?;
        }

        let now = Utc::now().to_rfc3339();
        app_conn
            .execute(
                "INSERT INTO db_metadata (key, value, created_at) VALUES (?1, ?2, ?3)
                 ON CONFLICT(key) DO UPDATE SET value = excluded.value, created_at = excluded.created_at",
                rusqlite::params!["last_trainee_gathered", now, now],
            )
            .map_err(|e| format!("metadata write: {e}"))?;

        Ok(())
    })
    .await
    .map_err(|e| format!("join: {e}"))?
}

#[tauri::command]
pub fn get_last_gather_time(key: String) -> Result<Option<String>, String> {
    let allowed = [
        "last_trainee_gathered",
        "last_support_card_gathered",
        "last_veterans_gathered",
    ];
    if !allowed.contains(&key.as_str()) {
        return Err("invalid key".into());
    }
    let conn = app_db::open_app_database_connection()?;
    conn.query_row(
        "SELECT value FROM db_metadata WHERE key = ?1",
        rusqlite::params![key],
        |row| row.get(0),
    )
    .optional()
    .map_err(|e| e.to_string())
}

fn await_frame(
    state: &WorkerState,
    request_id: u64,
    receiver: Receiver<Vec<u8>>,
    timeout_ms: u64,
) -> Result<Vec<u8>, String> {
    receiver
        .recv_timeout(Duration::from_millis(timeout_ms))
        .map_err(|_| {
            state.clear_pending(request_id);
            format!("timeout waiting for response for request {request_id}")
        })
}
