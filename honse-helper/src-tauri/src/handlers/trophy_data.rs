use crate::db::{app_db, schema::VeteranSchema};
use crate::worker::WorkerState;
use honse_worker::protocol::{
    parse_msgpack_frame_response, write_msgpack_request_framed, WorkerCommand, WorkerRequest,
};
use std::sync::mpsc::Receiver;
use std::time::Duration;
use tauri::{AppHandle, Manager, State};

const TROPHY_TIMEOUT_MS: u64 = 10_000;

#[tauri::command]
pub async fn import_trophy_data(
    app: AppHandle,
    mut request: WorkerRequest,
    timeout_ms: Option<u64>,
) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || {
        let state: State<'_, WorkerState> = app.state();

        if !matches!(request.command, WorkerCommand::GetTrophyData { .. }) {
            return Err("expected get_trophy_data command".to_string());
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
            timeout_ms.unwrap_or(TROPHY_TIMEOUT_MS),
        )?;

        let response =
            parse_msgpack_frame_response(&frame).ok_or("failed to parse response")?;

        let payload = match response {
            honse_worker::protocol::WorkerResponse::Ok(ok) => super::rmpv_to_json(ok.payload),
            honse_worker::protocol::WorkerResponse::Err(err) => return Err(err.error),
            _ => return Err("unexpected response type".to_string()),
        };

        let trophy_dic = payload
            .get("trophy_dic")
            .and_then(|v| v.as_array())
            .ok_or("payload missing trophy_dic")?;

        let tx = app_conn
            .transaction()
            .map_err(|e| format!("transaction: {e}"))?;

        tx.execute("DELETE FROM character_has_trophy", [])
            .map_err(|e| format!("delete: {e}"))?;

        let mut stmt = tx
            .prepare(
                "INSERT OR IGNORE INTO character_has_trophy (character_id, trophy_id) VALUES (?1, ?2)",
            )
            .map_err(|e| format!("prepare: {e}"))?;

        for entry in trophy_dic {
            let tid = entry["trophy_id"]
                .as_i64()
                .ok_or("entry missing trophy_id")?;
            let list = entry["chara_id_list"]
                .as_array()
                .ok_or("entry missing chara_id_list")?;
            for cid in list {
                if let Some(cid) = cid.as_i64() {
                    stmt.execute(rusqlite::params![cid, tid])
                        .map_err(|e| format!("insert: {e}"))?;
                }
            }
        }

        drop(stmt);
        tx.commit().map_err(|e| format!("commit: {e}"))?;

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
