use crate::worker::WorkerState;
use honse_worker::protocol::{
    parse_msgpack_frame_response, write_msgpack_request_framed, WorkerCommand, WorkerRequest,
    WorkerResponse,
};
use shared::worker_state::{WorkerConfig, WorkerStatusReport};
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use std::time::Duration;
use tauri::{AppHandle, Emitter, Manager, State};

static SUPERVISOR_ID: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1_000_000);

#[derive(Debug)]
struct WorkerStatusStateInner {
    config: WorkerConfig,
    process_found: bool,
    process_name: Option<String>,
    process_path: Option<String>,
    current_known_view: Option<String>,
    current_view_id_raw: Option<i32>,
    current_view_kclass: Option<String>,
    current_view_class: Option<String>,
    current_view_ptr: Option<String>,
    current_scene_base_ptr: Option<String>,
    current_scene_class: Option<String>,
    last_known_view: Option<String>,
    retry_count: u32,
}

impl WorkerStatusStateInner {
    fn new(config: WorkerConfig) -> Self {
        Self {
            config,
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
            retry_count: 0,
        }
    }

    fn snapshot(&self) -> WorkerStatusReport {
        WorkerStatusReport {
            worker_running: false,
            process_found: self.process_found,
            process_name: self.process_name.clone(),
            process_path: self.process_path.clone(),
            current_known_view: self.current_known_view.clone(),
            current_view_id_raw: self.current_view_id_raw,
            current_view_kclass: self.current_view_kclass.clone(),
            current_view_class: self.current_view_class.clone(),
            current_view_ptr: self.current_view_ptr.clone(),
            current_scene_base_ptr: self.current_scene_base_ptr.clone(),
            current_scene_class: self.current_scene_class.clone(),
            last_known_view: self.last_known_view.clone(),
            auto_start: self.config.auto_start,
            retry_count: self.retry_count,
            max_retries: self.config.max_retries,
            retry_interval_secs: self.config.retry_interval_secs,
            discovery_interval_secs: self.config.discovery_interval_secs,
        }
    }
}

pub struct WorkerStatusState {
    inner: Mutex<WorkerStatusStateInner>,
    supervisor_stop: AtomicBool,
}

impl WorkerStatusState {
    pub fn new() -> Self {
        let config = crate::app_config::worker_config();
        Self {
            inner: Mutex::new(WorkerStatusStateInner::new(config)),
            supervisor_stop: AtomicBool::new(false),
        }
    }

    pub(crate) fn auto_start_enabled(&self) -> bool {
        self.inner
            .lock()
            .map(|s| s.config.auto_start)
            .unwrap_or(true)
    }

    fn update_config(&self, config: WorkerConfig) {
        if let Ok(mut inner) = self.inner.lock() {
            inner.config = config;
            crate::app_config::update_worker_config(&inner.config);
        }
    }

    pub fn snapshot(&self, worker_running: bool) -> WorkerStatusReport {
        let mut report = if let Ok(inner) = self.inner.lock() {
            inner.snapshot()
        } else {
            WorkerStatusReport::default()
        };
        report.worker_running = worker_running;
        report
    }

    pub(crate) fn emit_status(&self, app: &AppHandle) {
        let worker_running = app.state::<WorkerState>().is_running().unwrap_or(false);
        let report = self.snapshot(worker_running);
        let _ = app.emit("worker-status", &report);
    }

    pub(crate) fn set_process_found(&self, found: bool, full_path: Option<String>) {
        if let Ok(mut inner) = self.inner.lock() {
            inner.process_found = found;
            inner.process_path = full_path.clone();
            inner.process_name = full_path
                .as_ref()
                .and_then(|p| {
                    Path::new(p)
                        .file_name()
                        .map(|n| n.to_string_lossy().to_string())
                        .or_else(|| {
                            let cleaned = p.replace('\\', "/");
                            Path::new(&cleaned)
                                .file_name()
                                .map(|n| n.to_string_lossy().to_string())
                        })
                })
                .or(full_path);
            if found {
                inner.retry_count = 0;
            }
        }
    }

    fn increment_retry(&self) {
        if let Ok(mut inner) = self.inner.lock() {
            inner.retry_count = inner.retry_count.saturating_add(1);
        }
    }

    pub(crate) fn reset_retry(&self) {
        if let Ok(mut inner) = self.inner.lock() {
            inner.retry_count = 0;
        }
    }

    pub(crate) fn reset_process_state(&self) {
        if let Ok(mut inner) = self.inner.lock() {
            inner.process_found = false;
            inner.process_name = None;
            inner.process_path = None;
            inner.retry_count = 0;
            inner.current_known_view = None;
            inner.current_view_id_raw = None;
            inner.current_view_kclass = None;
            inner.current_view_class = None;
            inner.current_view_ptr = None;
            inner.current_scene_base_ptr = None;
            inner.current_scene_class = None;
            inner.last_known_view = None;
        }
    }

    fn update_current_view(&self, payload: &serde_json::Value) {
        if let Ok(mut inner) = self.inner.lock() {
            inner.current_view_id_raw = payload
                .get("next_view_id")
                .and_then(|v| v.as_i64())
                .map(|v| v as i32)
                .or_else(|| {
                    payload
                        .get("next_view_id")
                        .and_then(|v| v.as_f64())
                        .map(|v| v as i32)
                });

            inner.current_view_kclass = payload.get("kclass").and_then(|v| {
                if v.is_null() || v.as_str().map(|s| s.is_empty()).unwrap_or(true) {
                    None
                } else {
                    v.as_str().map(|s| s.to_string())
                }
            });

            inner.current_view_class = payload.get("current_view_class").and_then(|v| {
                if v.is_null() || v.as_str().map(|s| s.is_empty()).unwrap_or(true) {
                    None
                } else {
                    v.as_str().map(|s| s.to_string())
                }
            });

            inner.current_view_ptr = payload.get("current_view_base_ptr").and_then(|v| {
                if v.is_null() {
                    None
                } else {
                    v.as_str().map(|s| s.to_string())
                }
            });

            inner.current_scene_base_ptr = payload.get("current_scene_base_ptr").and_then(|v| {
                if v.is_null() {
                    None
                } else {
                    v.as_str().map(|s| s.to_string())
                }
            });

            inner.current_scene_class = payload.get("current_scene_class").and_then(|v| {
                if v.is_null() || v.as_str().map(|s| s.is_empty()).unwrap_or(true) {
                    None
                } else {
                    v.as_str().map(|s| s.to_string())
                }
            });

            inner.last_known_view = payload.get("last_known_view").and_then(|v| {
                if v.is_null() {
                    None
                } else {
                    v.as_str().map(|s| s.to_string())
                }
            });

            inner.current_known_view = inner.last_known_view.clone();
        }
    }
}

impl Drop for WorkerStatusState {
    fn drop(&mut self) {
        self.supervisor_stop.store(true, Ordering::Relaxed);
    }
}

#[tauri::command]
pub fn get_worker_status(
    app: AppHandle,
    state: State<'_, WorkerStatusState>,
) -> Result<WorkerStatusReport, String> {
    let worker_running = app.state::<WorkerState>().is_running().unwrap_or(false);
    Ok(state.snapshot(worker_running))
}

#[tauri::command]
pub fn set_worker_auto_start(
    state: State<'_, WorkerStatusState>,
    app: AppHandle,
    auto_start: bool,
) -> Result<(), String> {
    let mut config = {
        let inner = state.inner.lock().map_err(|e| e.to_string())?;
        inner.config.clone()
    };
    config.auto_start = auto_start;
    state.update_config(config);
    state.emit_status(&app);
    Ok(())
}

#[tauri::command]
pub fn set_worker_retry_config(
    state: State<'_, WorkerStatusState>,
    app: AppHandle,
    max_retries: u32,
    interval_secs: u32,
) -> Result<(), String> {
    let mut config = {
        let inner = state.inner.lock().map_err(|e| e.to_string())?;
        inner.config.clone()
    };
    config.max_retries = max_retries;
    config.retry_interval_secs = interval_secs;
    state.update_config(config);
    state.emit_status(&app);
    Ok(())
}

#[tauri::command]
pub fn set_worker_discovery_interval(
    state: State<'_, WorkerStatusState>,
    app: AppHandle,
    interval_secs: u32,
) -> Result<(), String> {
    let mut config = {
        let inner = state.inner.lock().map_err(|e| e.to_string())?;
        inner.config.clone()
    };
    config.discovery_interval_secs = interval_secs;
    state.update_config(config);
    state.emit_status(&app);
    Ok(())
}

#[tauri::command]
pub fn reset_worker_retry_count(
    state: State<'_, WorkerStatusState>,
    app: AppHandle,
) -> Result<(), String> {
    state.reset_retry();
    state.emit_status(&app);
    Ok(())
}

pub fn spawn_worker_supervisor(app: AppHandle) {
    std::thread::spawn(move || {
        supervisor_loop(app);
    });
}

fn supervisor_loop(app: AppHandle) {
    let ws = app.state::<WorkerState>();
    let ss = app.state::<WorkerStatusState>();

    loop {
        if ss.supervisor_stop.load(Ordering::Relaxed) {
            break;
        }

        let worker_running = ws.is_running().unwrap_or(false);

        if !worker_running {
            ss.reset_process_state();
            ss.emit_status(&app);
            std::thread::sleep(Duration::from_secs(5));
            continue;
        }

        let process_found;
        let retry_exhausted;
        let retry_interval;
        let discovery_interval;
        {
            if let Ok(inner) = ss.inner.lock() {
                process_found = inner.process_found;
                retry_exhausted = inner.retry_count >= inner.config.max_retries;
                retry_interval = inner.config.retry_interval_secs;
                discovery_interval = inner.config.discovery_interval_secs;
            } else {
                std::thread::sleep(Duration::from_secs(5));
                continue;
            }
        }

        if !process_found && !retry_exhausted {
            let newly_found = supervisor_find_process(&app, &ws, &ss);
            if newly_found {
                supervisor_discover_view(&app, &ws, &ss);
            }
            std::thread::sleep(Duration::from_secs(retry_interval as u64));
        } else if process_found {
            supervisor_discover_view(&app, &ws, &ss);
            std::thread::sleep(Duration::from_secs(discovery_interval as u64));
        } else {
            if let Ok(Some(mut worker)) = ws.take_running() {
                let _ = write_msgpack_request_framed(
                    &mut worker.stdin,
                    &WorkerRequest {
                        id: None,
                        command: WorkerCommand::Quit,
                    },
                );
                let _ = worker.child.kill();
                let _ = worker.child.wait();
            }
            ss.reset_process_state();
            ss.emit_status(&app);
            break;
        }
    }
}

/// Returns true if process was newly found on this call
fn supervisor_find_process(app: &AppHandle, ws: &WorkerState, ss: &WorkerStatusState) -> bool {
    let was_found = ss.inner.lock().map(|i| i.process_found).unwrap_or(false);

    let id = SUPERVISOR_ID.fetch_add(1, Ordering::Relaxed);
    let rx = match ws.register_pending(id) {
        Ok(rx) => rx,
        Err(_) => return false,
    };

    let write_ok = ws.with_running_worker(|running| {
        write_msgpack_request_framed(
            &mut running.stdin,
            &WorkerRequest {
                id: Some(id),
                command: WorkerCommand::FindProcess,
            },
        )
        .map_err(|e| e.to_string())
    });

    if write_ok.is_err() {
        ws.clear_pending(id);
        ss.increment_retry();
        ss.emit_status(app);
        return false;
    }

    match rx.recv_timeout(Duration::from_secs(10)) {
        Ok(frame) => {
            if let Some(response) = parse_msgpack_frame_response(&frame) {
                match response {
                    WorkerResponse::Ok(ok_resp) => {
                        let payload = super::rmpv_to_json(ok_resp.payload);
                        let pid_exists = payload
                            .get("pid")
                            .and_then(|v| v.as_u64())
                            .map(|p| p > 0)
                            .unwrap_or(false);

                        if pid_exists {
                            let full_path = payload
                                .get("path")
                                .and_then(|v| v.as_str())
                                .or_else(|| payload.get("name").and_then(|v| v.as_str()))
                                .map(|s| s.to_string());

                            ss.set_process_found(true, full_path);
                            ss.emit_status(app);
                            return !was_found;
                        } else {
                            ss.increment_retry();
                        }
                    }
                    _ => {
                        ss.increment_retry();
                    }
                }
            } else {
                ss.increment_retry();
            }
        }
        Err(_) => {
            ws.clear_pending(id);
            ss.increment_retry();
        }
    }

    ss.emit_status(app);
    false
}

pub(crate) fn supervisor_discover_view(app: &AppHandle, ws: &WorkerState, ss: &WorkerStatusState) {
    let id = SUPERVISOR_ID.fetch_add(1, Ordering::Relaxed);
    let rx = match ws.register_pending(id) {
        Ok(rx) => rx,
        Err(_) => return,
    };

    let write_ok = ws.with_running_worker(|running| {
        write_msgpack_request_framed(
            &mut running.stdin,
            &WorkerRequest {
                id: Some(id),
                command: WorkerCommand::GetViewState {
                    max_scan_bytes: None,
                },
            },
        )
        .map_err(|e| e.to_string())
    });

    if write_ok.is_err() {
        ws.clear_pending(id);
        ss.emit_status(app);
        return;
    }

    match rx.recv_timeout(Duration::from_secs(10)) {
        Ok(frame) => {
            if let Some(response) = parse_msgpack_frame_response(&frame) {
                if let WorkerResponse::Ok(ok_resp) = response {
                    ss.update_current_view(&super::rmpv_to_json(ok_resp.payload));
                }
            }
        }
        Err(_) => {
            ws.clear_pending(id);
        }
    }

    ss.emit_status(app);
}
