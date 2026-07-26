use crate::db::{app_db, schema::VeteranSchema};
use crate::worker::WorkerState;
use chrono::Utc;
use honse_worker::protocol::{
    parse_msgpack_frame_response, write_msgpack_request_framed, WorkerCommand, WorkerRequest,
};
use std::sync::mpsc::Receiver;
use std::time::Duration;
use tauri::{AppHandle, Manager, State};

const SC_TIMEOUT_MS: u64 = 10_000;

#[tauri::command]
pub async fn import_support_card_data(
    app: AppHandle,
    mut request: WorkerRequest,
    timeout_ms: Option<u64>,
) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || {
        let state: State<'_, WorkerState> = app.state();

        if !matches!(request.command, WorkerCommand::GetSupportCardData { .. }) {
            return Err("expected get_support_card_data command".to_string());
        }

        let request_id = request.id.unwrap_or_else(|| state.next_request_id());
        request.id = Some(request_id);

        let mut app_conn = app_db::open_app_database_connection()?;
        VeteranSchema::ensure_current(&app_conn).map_err(|e| format!("schema init: {e}"))?;

        let receiver = state.register_pending(request_id)?;

        state.with_running_worker(|running| {
            write_msgpack_request_framed(&mut running.stdin, &request)
                .map_err(|e| format!("write failed: {e}"))
        })?;

        let frame = await_frame(
            &state,
            request_id,
            receiver,
            timeout_ms.unwrap_or(SC_TIMEOUT_MS),
        )?;

        let response = parse_msgpack_frame_response(&frame).ok_or("failed to parse response")?;

        let payload = match response {
            honse_worker::protocol::WorkerResponse::Ok(ok) => super::rmpv_to_json(ok.payload),
            honse_worker::protocol::WorkerResponse::Err(err) => return Err(err.error),
            _ => return Err("unexpected response type".to_string()),
        };

        let cards = payload
            .get("support_cards")
            .and_then(|v| v.as_array())
            .ok_or("payload missing support_cards")?;

        let tx = app_conn
            .transaction()
            .map_err(|e| format!("transaction: {e}"))?;

        let mut stmt = tx
            .prepare(
                r#"INSERT OR REPLACE INTO support_card_owned
                    (support_card_id, level, max_level, exp, limit_break_count,
                     favorite_flag, stock, create_time, possess_time, best_training)
                    VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)"#,
            )
            .map_err(|e| format!("prepare: {e}"))?;

        for entry in cards {
            let id = entry["support_card_id"]
                .as_i64()
                .ok_or("entry missing support_card_id")?;
            let level = entry["level"].as_i64().unwrap_or(0);
            let max_level = entry["max_level"].as_i64().unwrap_or(0);
            let exp = entry["exp"].as_i64().unwrap_or(0);
            let lb = entry["limit_break_count"].as_i64().unwrap_or(0);
            let fav = entry["favorite_flag"].as_i64().unwrap_or(0);
            let stock = entry["stock"].as_i64().unwrap_or(0);
            let ct = entry["create_time"].as_i64().unwrap_or(0);
            let pt = entry["possess_time"].as_i64().unwrap_or(0);
            let bt = entry["best_training"].as_i64().unwrap_or(0);

            stmt.execute(rusqlite::params![
                id, level, max_level, exp, lb, fav, stock, ct, pt, bt
            ])
            .map_err(|e| format!("insert: {e}"))?;
        }

        drop(stmt);
        tx.commit().map_err(|e| format!("commit: {e}"))?;

        let now = Utc::now().to_rfc3339();
        app_conn
            .execute(
                "INSERT INTO db_metadata (key, value, created_at) VALUES (?1, ?2, ?3)
                 ON CONFLICT(key) DO UPDATE SET value = excluded.value, created_at = excluded.created_at",
                rusqlite::params!["last_support_card_gathered", now, now],
            )
            .map_err(|e| format!("metadata write: {e}"))?;

        Ok(())
    })
    .await
    .map_err(|e| format!("join: {e}"))?
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
