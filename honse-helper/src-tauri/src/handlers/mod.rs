use crate::worker::WorkerState;
use honse_worker::protocol::{
    parse_msgpack_frame_id, parse_msgpack_frame_response, read_msgpack_frame_raw,
    write_msgpack_request_framed, WorkerCommand, WorkerRequest, WorkerResponse,
};
use std::fs;
use std::io::BufRead;
use std::io::BufReader;
use std::process::{Child, Command, Stdio};
use tauri::{AppHandle, Emitter, Manager, State};

use self::worker::WorkerStatusState;

pub mod api_config;
pub mod card_data;
pub mod legacy_planner;
pub mod race_dump;
pub mod supplementary_data;
pub mod support_card_browser;
pub mod support_card_data;
pub mod tags;
pub mod trainee_browser;
pub mod trophy_data;
pub mod veteran_browser;
pub mod veterans;
pub mod worker;

/// Spawns the honse_worker sidecar process with the specified protocol
#[tauri::command]
pub fn start_worker(app: AppHandle, use_pkexec: Option<bool>) -> Result<(), String> {
    if use_pkexec.unwrap_or(false) {
        start_worker_with_pkexec(app)
    } else {
        start_worker_inner(&app)
    }
}

/// Sends a command request to the running worker
#[tauri::command]
pub fn send_worker_request(
    state: State<'_, WorkerState>,
    request: WorkerRequest,
) -> Result<(), String> {
    state.with_running_worker(|running| {
        write_msgpack_request_framed(&mut running.stdin, &request)
            .map_err(|err| format!("failed to write msgpack request: {err}"))
    })
}

/// Gracefully stops the worker process
#[tauri::command]
pub fn stop_worker(app: AppHandle, state: State<'_, WorkerState>) -> Result<(), String> {
    if let Some(mut worker) = state.take_running()? {
        let _ = write_msgpack_request_framed(
            &mut worker.stdin,
            &WorkerRequest {
                id: None,
                command: WorkerCommand::Quit,
            },
        );
        worker
            .child
            .kill()
            .map_err(|e| format!("failed to kill worker: {e}"))?;
        let _ = worker.child.wait();
    }

    let status_state = app.state::<WorkerStatusState>();
    status_state.reset_process_state();
    status_state.emit_status(&app);
    Ok(())
}

/// Checks if the worker process is currently running
#[tauri::command]
pub fn is_worker_running(state: State<'_, WorkerState>) -> Result<bool, String> {
    state.is_running()
}

/// Opens a native save dialog and writes the JSON response to disk.
#[tauri::command]
pub fn save_worker_response(json: String) -> Result<String, String> {
    let default_name = format!(
        "worker-response-{}.json",
        chrono::Local::now().format("%Y%m%d-%H%M%S")
    );

    let Some(path) = rfd::FileDialog::new()
        .add_filter("JSON", &["json"])
        .set_file_name(&default_name)
        .save_file()
    else {
        return Ok("canceled".to_string());
    };

    fs::write(&path, json)
        .map_err(|e| format!("failed to write file '{}': {e}", path.display()))?;

    Ok(path.display().to_string())
}

fn resolve_worker_binary() -> String {
    std::env::var("HONSE_WORKER_BIN").unwrap_or_else(|_| {
        let file_name = if cfg!(target_os = "windows") {
            "honse-worker.exe"
        } else {
            "honse-worker"
        };

        if let Ok(exe) = std::env::current_exe() {
            if let Some(dir) = exe.parent() {
                let candidate = dir.join(file_name);
                if candidate.exists() {
                    return candidate.display().to_string();
                }
            }
        }

        file_name.to_string()
    })
}

fn spawn_worker_process(worker_bin: &str, use_pkexec: bool) -> Result<Child, String> {
    let mut command = build_worker_command(worker_bin, use_pkexec);
    command
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    command
        .spawn()
        .map_err(|err| format!("failed to spawn worker '{worker_bin}': {err}"))
}

fn build_worker_command(worker_bin: &str, use_pkexec: bool) -> Command {
    if cfg!(target_os = "linux") && use_pkexec {
        let mut command = Command::new("pkexec");
        command.arg(worker_bin);
        command
    } else {
        #[cfg(not(target_os = "windows"))]
        let command = Command::new(worker_bin);

        #[cfg(target_os = "windows")]
        let command = {
            let mut command = Command::new(worker_bin);
            use std::os::windows::process::CommandExt;
            command.creation_flags(0x08000000);
            command
        };

        command
    }
}

fn spawn_stdout_reader(app: AppHandle, stdout: std::process::ChildStdout, pid: u32) {
    std::thread::spawn(move || {
        let mut reader = BufReader::new(stdout);
        loop {
            match read_msgpack_frame_raw(&mut reader) {
                Ok(Some(frame)) => forward_worker_frame(&app, frame),
                Ok(None) => break,
                Err(err) => {
                    let _ = app.emit("worker-stream-error", format!("msgpack read error: {err}"));
                    break;
                }
            }
        }
        clear_worker_state_if_pid_matches(&app, pid);
        let _ = app.emit("worker-exit", "stdout closed");
    });
}

fn forward_worker_frame(app: &AppHandle, frame: Vec<u8>) {
    let consumed_by_pending = parse_msgpack_frame_id(&frame)
        .map(|request_id| {
            let state = app.state::<WorkerState>();
            state.resolve_pending_raw(request_id, frame.clone())
        })
        .unwrap_or(false);

    if !consumed_by_pending {
        if let Some(response) = parse_msgpack_frame_response(&frame) {
            if let WorkerResponse::Ok(ref ok_resp) = response {
                let payload = rmpv_to_json(ok_resp.payload.clone());
                let pid_exists = payload
                    .get("pid")
                    .and_then(|v| v.as_u64())
                    .map(|p| p > 0)
                    .unwrap_or(false);
                if pid_exists {
                    let ss = app.state::<WorkerStatusState>();
                    let full_path = payload
                        .get("path")
                        .and_then(|v| v.as_str())
                        .or_else(|| payload.get("name").and_then(|v| v.as_str()))
                        .map(|s| s.to_string());
                    ss.set_process_found(true, full_path);
                    ss.emit_status(app);
                }
            }
            let _ = app.emit("worker-response", response);
        }
    }
}

fn clear_worker_state_if_pid_matches(app: &AppHandle, pid: u32) {
    let state = app.state::<WorkerState>();
    state.clear_if_pid_matches(pid);
    let status_state = app.state::<WorkerStatusState>();
    status_state.reset_process_state();
    status_state.emit_status(app);
}

fn spawn_stderr_reader(app: AppHandle, stderr: std::process::ChildStderr) {
    std::thread::spawn(move || {
        let mut reader = BufReader::new(stderr);
        let mut line = String::new();
        loop {
            line.clear();
            match reader.read_line(&mut line) {
                Ok(0) => break,
                Ok(_) => {
                    let msg = line.trim();
                    if !msg.is_empty() {
                        let _ = app.emit("worker-stderr", msg.to_string());
                    }
                }
                Err(err) => {
                    let _ = app.emit("worker-stream-error", format!("stderr read error: {err}"));
                    break;
                }
            }
        }
    });
}

pub(crate) fn start_worker_inner(app: &AppHandle) -> Result<(), String> {
    let ws = app.state::<WorkerState>();
    if ws.is_running()? {
        return Ok(());
    }

    let worker_bin = resolve_worker_binary();
    let child = spawn_worker_process(&worker_bin, false)?;
    start_worker_post_spawn(app, child)
}

fn start_worker_with_pkexec(app: AppHandle) -> Result<(), String> {
    let ws = app.state::<WorkerState>();
    if ws.is_running()? {
        return Err("worker already running".to_string());
    }
    let worker_bin = resolve_worker_binary();
    let child = spawn_worker_process(&worker_bin, true)?;
    start_worker_post_spawn(&app, child)
}

fn start_worker_post_spawn(app: &AppHandle, mut child: Child) -> Result<(), String> {
    let ws = app.state::<WorkerState>();
    let pid = child.id();

    let stdin = child
        .stdin
        .take()
        .ok_or_else(|| "failed to take worker stdin".to_string())?;
    let stdout = child
        .stdout
        .take()
        .ok_or_else(|| "failed to take worker stdout".to_string())?;
    let stderr = child
        .stderr
        .take()
        .ok_or_else(|| "failed to take worker stderr".to_string())?;

    spawn_stdout_reader(app.clone(), stdout, pid);
    spawn_stderr_reader(app.clone(), stderr);

    ws.store_running(crate::worker::RunningWorker { child, stdin, pid })?;

    app.emit("worker-started", pid).map_err(|e| e.to_string())?;

    let status_state = app.state::<WorkerStatusState>();
    status_state.reset_retry();
    status_state.emit_status(app);
    Ok(())
}

pub(crate) fn rmpv_to_json(v: rmpv::Value) -> serde_json::Value {
    serde_json::to_value(v).expect("rmpv::Value is always JSON-compatible")
}
